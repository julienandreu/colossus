//! Workflow execution engine
//!
//! This module provides the core functionality for executing workflows.
//! It handles file parsing, format detection, and workflow execution
//! with comprehensive error handling.

use std::path::PathBuf;

use tracing::error;

use crate::core::heap::Heap;
use crate::nodes::base::BaseNodeRunOptions;
use crate::nodes::NodeBuilder;
use crate::shared::types::workflow::workflow::Workflow;

/// Configuration options for workflow execution
///
/// This struct encapsulates all the configuration needed to execute a workflow,
/// providing a clean interface for specifying workflow parameters.
#[derive(Debug, Clone)]
pub struct ExecuteWorkflowOptions {
    /// Path to the workflow file
    path: PathBuf,
}

impl ExecuteWorkflowOptions {
    /// Creates a new `ExecuteWorkflowOptions` with the specified file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the workflow file (can be any type that converts to PathBuf)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use colossus::core::engine::ExecuteWorkflowOptions;
    ///
    /// let options = ExecuteWorkflowOptions::new("workflow.yml");
    /// let options = ExecuteWorkflowOptions::new(PathBuf::from("workflow.yml"));
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    /// Returns a reference to the workflow file path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Builder method to set the workflow file path
    ///
    /// This method allows for fluent API usage when building options.
    ///
    /// # Arguments
    ///
    /// * `path` - New path to set
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::engine::ExecuteWorkflowOptions;
    ///
    /// let options = ExecuteWorkflowOptions::default()
    ///     .with_path("custom-workflow.yml");
    /// ```
    pub fn with_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }
}

impl Default for ExecuteWorkflowOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("workflow.yml"),
        }
    }
}

/// Errors that can occur during workflow execution
///
/// This enum provides comprehensive error handling for all possible
/// failure modes in workflow execution, with detailed error messages
/// and proper error chaining.
#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    /// Failed to read the workflow file from disk
    #[error("Failed to read workflow file: {0}")]
    FileRead(#[from] std::io::Error),

    /// Failed to parse JSON workflow file
    #[error("Failed to parse JSON workflow file: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Failed to parse YAML workflow file
    #[error("Failed to parse YAML workflow file: {0}")]
    YamlParse(#[from] serde_yml::Error),

    /// Unsupported file format
    #[error("Unsupported file format. Expected .json, .yml, or .yaml extension")]
    UnsupportedFormat,

    /// Workflow file not found
    #[error("Workflow file not found: {path}")]
    NotFound { path: PathBuf },

    /// Node builder error
    #[error("Node builder error: {0}")]
    NodeBuilder(String),

    /// Node execution failed
    #[error("Node execution failed: {0}")]
    NodeExecutionFailed(String),

    /// Invalid node type
    #[error("Invalid node type: {0}")]
    InvalidNode(String),
}

/// Result type for workflow operations
///
/// This type alias provides a convenient way to return results from
/// workflow operations with proper error handling.
pub type WorkflowResult<T> = Result<T, WorkflowError>;

/// Supported file formats for workflow files
///
/// This enum represents the different file formats that can be parsed
/// by the workflow engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// JSON format
    Json,
    /// YAML format (both .yml and .yaml extensions)
    Yaml,
}

impl FileFormat {
    /// Determines the file format based on the file extension
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the workflow file
    ///
    /// # Returns
    ///
    /// Returns `Some(FileFormat)` if the format is supported, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use colossus::core::engine::FileFormat;
    ///
    /// let json_path = PathBuf::from("workflow.json");
    /// assert_eq!(FileFormat::from_path(&json_path), Some(FileFormat::Json));
    ///
    /// let yaml_path = PathBuf::from("workflow.yml");
    /// assert_eq!(FileFormat::from_path(&yaml_path), Some(FileFormat::Yaml));
    ///
    /// let txt_path = PathBuf::from("workflow.txt");
    /// assert_eq!(FileFormat::from_path(&txt_path), None);
    /// ```
    pub fn from_path(path: &PathBuf) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext.to_lowercase().as_str() {
                "json" => Some(FileFormat::Json),
                "yml" | "yaml" => Some(FileFormat::Yaml),
                _ => None,
            })
    }

    /// Parses workflow content based on the format
    ///
    /// # Arguments
    ///
    /// * `content` - The file content as a string
    ///
    /// # Returns
    ///
    /// Returns a `WorkflowResult` containing the parsed workflow or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::engine::{FileFormat, WorkflowResult};
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let yaml_content = r#"
    /// name: "Test Workflow"
    /// version: "1.0.0"
    /// "#;
    ///
    /// match FileFormat::Yaml.parse_content(yaml_content) {
    ///     Ok(workflow) => println!("Parsed workflow: {:?}", workflow),
    ///     Err(e) => eprintln!("Parse error: {}", e),
    /// }
    /// ```
    pub fn parse_content(self, content: &str) -> WorkflowResult<Workflow> {
        match self {
            FileFormat::Json => serde_json::from_str(content).map_err(WorkflowError::JsonParse),
            FileFormat::Yaml => serde_yml::from_str(content).map_err(WorkflowError::YamlParse),
        }
    }
}

/// Workflow executor that handles the execution logic
///
/// This struct encapsulates the workflow execution logic, making it
/// easier to test and maintain.
#[derive(Debug)]
pub struct WorkflowExecutor;

impl WorkflowExecutor {
    /// Executes a workflow from the given options
    ///
    /// This function reads and parses a workflow file, supporting both JSON and YAML formats.
    /// The format is automatically detected based on the file extension.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for workflow execution
    /// * `heap` - The heap containing shared data for the workflow execution
    ///
    /// # Returns
    ///
    /// Returns a `WorkflowResult` containing the parsed workflow or an error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::engine::{ExecuteWorkflowOptions, WorkflowExecutor};
    /// use colossus::core::heap::Heap;
    ///
    /// let options = ExecuteWorkflowOptions::new("workflows/simple-log.yml");
    /// let mut heap = Heap::new();
    /// match WorkflowExecutor::execute(options, &mut heap) {
    ///     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
    ///     Err(e) => eprintln!("Failed to execute workflow: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file does not exist
    /// - The file format is not supported
    /// - The file cannot be read
    /// - The file content cannot be parsed
    pub fn execute(options: ExecuteWorkflowOptions, heap: &mut Heap) -> WorkflowResult<Workflow> {
        let path = options.path();

        // Check if file exists
        if !path.exists() {
            return Err(WorkflowError::NotFound { path: path.clone() });
        }

        // Determine file format
        let format = FileFormat::from_path(path).ok_or(WorkflowError::UnsupportedFormat)?;

        // Read and parse the workflow file
        let content = std::fs::read_to_string(path).map_err(WorkflowError::FileRead)?;
        let workflow = format.parse_content(&content)?;

        // Execute the workflow nodes
        Self::execute_nodes(&workflow, heap)?;

        Ok(workflow)
    }

    /// Executes all nodes in a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to execute
    /// * `heap` - The heap containing shared data
    ///
    /// # Returns
    ///
    /// Returns a `WorkflowResult` indicating success or failure
    fn execute_nodes(workflow: &Workflow, heap: &mut Heap) -> WorkflowResult<()> {
        if let Some(nodes) = &workflow.nodes {
            for node in nodes {
                let key = node.id.clone();

                let node_instance = NodeBuilder::new()
                    .with_workflow_node(node.clone())
                    .build(heap)
                    .map_err(|e| WorkflowError::NodeBuilder(e.to_string()))?;

                let output = node_instance.execute(BaseNodeRunOptions::new(heap, key.clone()));

                if let Err(e) = &output {
                    error!("Node '{}' execution failed: {:?}", key, e);
                }

                heap.insert(key, output.ok());
            }
        }

        Ok(())
    }
}

/// Convenience function to execute a workflow from a string path
///
/// This function provides a more ergonomic API for simple use cases.
///
/// # Arguments
///
/// * `path` - Path to the workflow file as a string
/// * `heap` - The heap containing shared data
///
/// # Returns
///
/// Returns a `WorkflowResult` containing the parsed workflow or an error
///
/// # Examples
///
/// ```rust
/// use colossus::core::engine::execute_workflow_from_path;
/// use colossus::core::heap::Heap;
///
/// let mut heap = Heap::new();
/// match execute_workflow_from_path("workflow.yml", &mut heap) {
///     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
///     Err(e) => eprintln!("Failed to load workflow: {}", e),
/// }
/// ```
pub fn execute_workflow_from_path<P: AsRef<str>>(
    path: P,
    heap: &mut Heap,
) -> WorkflowResult<Workflow> {
    let options = ExecuteWorkflowOptions::new(path.as_ref());
    WorkflowExecutor::execute(options, heap)
}

/// Convenience function to execute a workflow from a PathBuf
///
/// This function provides a more ergonomic API for simple use cases.
///
/// # Arguments
///
/// * `path` - Path to the workflow file
/// * `heap` - The heap containing shared data
///
/// # Returns
///
/// Returns a `WorkflowResult` containing the parsed workflow or an error
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use colossus::core::engine::execute_workflow_from_pathbuf;
/// use colossus::core::heap::Heap;
///
/// let path = PathBuf::from("workflow.yml");
/// let mut heap = Heap::new();
/// match execute_workflow_from_pathbuf(path, &mut heap) {
///     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
///     Err(e) => eprintln!("Failed to load workflow: {}", e),
/// }
/// ```
pub fn execute_workflow_from_pathbuf<P: Into<PathBuf>>(
    path: P,
    heap: &mut Heap,
) -> WorkflowResult<Workflow> {
    let options = ExecuteWorkflowOptions::new(path);
    WorkflowExecutor::execute(options, heap)
}

// Legacy function for backward compatibility
#[deprecated(since = "0.2.0", note = "Use WorkflowExecutor::execute instead")]
pub fn execute_workflow(
    options: ExecuteWorkflowOptions,
    heap: &mut Heap,
) -> WorkflowResult<Workflow> {
    WorkflowExecutor::execute(options, heap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_execute_workflow_options_new() {
        let options = ExecuteWorkflowOptions::new("test.yml");
        assert_eq!(options.path(), &PathBuf::from("test.yml"));
    }

    #[test]
    fn test_execute_workflow_options_with_path() {
        let options = ExecuteWorkflowOptions::new("initial.yml").with_path("updated.yml");
        assert_eq!(options.path(), &PathBuf::from("updated.yml"));
    }

    #[test]
    fn test_execute_workflow_options_default() {
        let options = ExecuteWorkflowOptions::default();
        assert_eq!(options.path(), &PathBuf::from("workflow.yml"));
    }

    #[test]
    fn test_file_format_from_path() {
        // Test JSON format
        let json_path = PathBuf::from("workflow.json");
        assert_eq!(FileFormat::from_path(&json_path), Some(FileFormat::Json));

        // Test YAML formats
        let yml_path = PathBuf::from("workflow.yml");
        assert_eq!(FileFormat::from_path(&yml_path), Some(FileFormat::Yaml));

        let yaml_path = PathBuf::from("workflow.yaml");
        assert_eq!(FileFormat::from_path(&yaml_path), Some(FileFormat::Yaml));

        // Test unsupported formats
        let txt_path = PathBuf::from("workflow.txt");
        assert_eq!(FileFormat::from_path(&txt_path), None);

        let no_ext_path = PathBuf::from("workflow");
        assert_eq!(FileFormat::from_path(&no_ext_path), None);
    }

    #[test]
    fn test_file_format_parse_content_yaml() {
        let yaml_content = r#"
name: "Test Workflow"
version: "1.0.0"
nodes:
  - id: "log1"
    type: "Log"
    input: "Hello, World!"
"#;

        let result = FileFormat::Yaml.parse_content(yaml_content);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
        assert!(workflow.nodes.is_some());
    }

    #[test]
    fn test_file_format_parse_content_json() {
        let json_content = r#"{
            "name": "Test Workflow",
            "version": "1.0.0",
            "nodes": [
                {
                    "id": "log1",
                    "type": "Log",
                    "input": "Hello, World!"
                }
            ]
        }"#;

        let result = FileFormat::Json.parse_content(json_content);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
        assert!(workflow.nodes.is_some());
    }

    #[test]
    fn test_file_format_parse_invalid_yaml() {
        let invalid_yaml = r#"
name: "Test Workflow"
version: "1.0.0"
nodes:
  - id: "log1"
    type: "Log"
    input: "Hello, World!"
    invalid: [unclosed bracket
"#;

        let result = FileFormat::Yaml.parse_content(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_format_parse_invalid_json() {
        let invalid_json = r#"{
            "name": "Test Workflow",
            "version": "1.0.0",
            "nodes": [
                {
                    "id": "log1",
                    "type": "Log",
                    "input": "Hello, World!"
                }
            ]
        "#; // Missing closing brace

        let result = FileFormat::Json.parse_content(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_workflow_executor_file_not_found() {
        let options = ExecuteWorkflowOptions::new("nonexistent.yml");
        let mut heap = Heap::new();

        let result = WorkflowExecutor::execute(options, &mut heap);
        assert!(result.is_err());

        match result.unwrap_err() {
            WorkflowError::NotFound { path } => {
                assert_eq!(path, PathBuf::from("nonexistent.yml"));
            }
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_workflow_executor_unsupported_format() {
        // Create a temporary file with unsupported extension
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().with_extension("txt");
        std::fs::write(&path, "some content").unwrap();

        let options = ExecuteWorkflowOptions::new(path.clone());
        let mut heap = Heap::new();

        let result = WorkflowExecutor::execute(options, &mut heap);
        assert!(result.is_err());

        match result.unwrap_err() {
            WorkflowError::UnsupportedFormat => {
                // Expected error
            }
            _ => panic!("Expected UnsupportedFormat error"),
        }

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_workflow_executor_valid_yaml() {
        let yaml_content = r#"
name: "Test Workflow"
version: "1.0.0"
nodes:
  - id: "log1"
    type: "Log"
    input: "Hello, World!"
"#;

        // Create a temporary YAML file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        let path = temp_file.path().with_extension("yml");
        std::fs::rename(temp_file.path(), &path).unwrap();

        let options = ExecuteWorkflowOptions::new(path.clone());
        let mut heap = Heap::new();

        let result = WorkflowExecutor::execute(options, &mut heap);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_workflow_executor_valid_json() {
        let json_content = r#"{
            "name": "Test Workflow",
            "version": "1.0.0",
            "nodes": [
                {
                    "id": "log1",
                    "type": "Log",
                    "input": "Hello, World!"
                }
            ]
        }"#;

        // Create a temporary JSON file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();
        let path = temp_file.path().with_extension("json");
        std::fs::rename(temp_file.path(), &path).unwrap();

        let options = ExecuteWorkflowOptions::new(path.clone());
        let mut heap = Heap::new();

        let result = WorkflowExecutor::execute(options, &mut heap);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_execute_workflow_from_path() {
        let yaml_content = r#"
name: "Test Workflow"
version: "1.0.0"
"#;

        // Create a temporary YAML file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        let path = temp_file.path().with_extension("yml");
        std::fs::rename(temp_file.path(), &path).unwrap();

        let mut heap = Heap::new();
        let result = execute_workflow_from_path(path.to_str().unwrap(), &mut heap);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_execute_workflow_from_pathbuf() {
        let yaml_content = r#"
name: "Test Workflow"
version: "1.0.0"
"#;

        // Create a temporary YAML file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        let path = temp_file.path().with_extension("yml");
        std::fs::rename(temp_file.path(), &path).unwrap();

        let mut heap = Heap::new();
        let result = execute_workflow_from_pathbuf(path.clone(), &mut heap);
        assert!(result.is_ok());

        let workflow = result.unwrap();
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_workflow_error_display() {
        let error = WorkflowError::InvalidNode("UnknownNode".to_string());
        assert!(error.to_string().contains("Invalid node type: UnknownNode"));

        let error = WorkflowError::NodeExecutionFailed("Test error".to_string());
        assert!(error
            .to_string()
            .contains("Node execution failed: Test error"));

        let error = WorkflowError::NodeBuilder("Builder error".to_string());
        assert!(error
            .to_string()
            .contains("Node builder error: Builder error"));
    }
}
