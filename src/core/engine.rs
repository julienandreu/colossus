//! Workflow execution engine
//!
//! This module provides the core functionality for executing workflows.
//! It handles file parsing, format detection, and workflow execution
//! with comprehensive error handling.

use std::path::PathBuf;
use std::str::FromStr;

use crate::shared::types::workflow::workflow::Workflow;

/// Configuration options for workflow execution
///
/// This struct encapsulates all the configuration needed to execute a workflow,
/// providing a clean interface for specifying workflow parameters.
#[derive(Debug, Clone)]
pub struct WorkflowOptions {
    /// Path to the workflow file
    path: PathBuf,
}

impl WorkflowOptions {
    /// Creates a new `WorkflowOptions` with the specified file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the workflow file (can be any type that converts to PathBuf)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use colossus::core::engine::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::new("workflow.yml");
    /// let options = WorkflowOptions::new(PathBuf::from("workflow.yml"));
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    /// Returns a reference to the workflow file path
    ///
    /// # Returns
    ///
    /// Returns a reference to the internal PathBuf.
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
    /// use colossus::core::engine::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::default()
    ///     .with_path("custom-workflow.yml");
    /// ```
    pub fn with_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }
}

impl Default for WorkflowOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("workflow.yml"),
        }
    }
}

impl FromStr for WorkflowOptions {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<PathBuf> for WorkflowOptions {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&str> for WorkflowOptions {
    fn from(path: &str) -> Self {
        Self::new(path)
    }
}

impl From<String> for WorkflowOptions {
    fn from(path: String) -> Self {
        Self::new(path)
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
#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    /// JSON format
    Json,
    /// YAML format (both .yml and .yaml extensions)
    Yaml,
}

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
/// use colossus::core::engine::get_file_format;
///
/// let json_path = PathBuf::from("workflow.json");
/// assert!(get_file_format(&json_path).is_some());
///
/// let yaml_path = PathBuf::from("workflow.yml");
/// assert!(get_file_format(&yaml_path).is_some());
///
/// let txt_path = PathBuf::from("workflow.txt");
/// assert!(get_file_format(&txt_path).is_none());
/// ```
pub fn get_file_format(path: &PathBuf) -> Option<FileFormat> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| match ext.to_lowercase().as_str() {
            "json" => Some(FileFormat::Json),
            "yml" | "yaml" => Some(FileFormat::Yaml),
            _ => None,
        })
}

/// Parses workflow content based on the specified format
///
/// # Arguments
///
/// * `content` - The file content as a string
/// * `format` - The format of the content
///
/// # Returns
///
/// Returns a `WorkflowResult` containing the parsed workflow or an error
///
/// # Examples
///
/// ```rust
/// use colossus::core::engine::{parse_workflow_content, FileFormat};
///
/// let yaml_content = r#"
/// name: "Test Workflow"
/// version: "1.0.0"
/// "#;
///
/// match parse_workflow_content(yaml_content, FileFormat::Yaml) {
///     Ok(workflow) => println!("Parsed workflow: {:?}", workflow),
///     Err(e) => eprintln!("Parse error: {}", e),
/// }
/// ```
pub fn parse_workflow_content(content: &str, format: FileFormat) -> WorkflowResult<Workflow> {
    match format {
        FileFormat::Json => serde_json::from_str(content).map_err(WorkflowError::JsonParse),
        FileFormat::Yaml => serde_yml::from_str(content).map_err(WorkflowError::YamlParse),
    }
}

/// Executes a workflow from the given options
///
/// This function reads and parses a workflow file, supporting both JSON and YAML formats.
/// The format is automatically detected based on the file extension.
///
/// # Arguments
///
/// * `options` - Configuration options for workflow execution
///
/// # Returns
///
/// Returns a `WorkflowResult` containing the parsed workflow or an error
///
/// # Examples
///
/// ```rust
/// use colossus::core::engine::{WorkflowOptions, execute_workflow};
///
/// let options = WorkflowOptions::new("workflows/simple-log.yml");
/// match execute_workflow(options) {
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
pub fn execute_workflow(options: WorkflowOptions) -> WorkflowResult<Workflow> {
    let path = options.path();

    // Check if file exists
    if !path.exists() {
        return Err(WorkflowError::NotFound { path: path.clone() });
    }

    // Determine file format
    let format = get_file_format(path).ok_or(WorkflowError::UnsupportedFormat)?;

    // Read and parse the workflow file
    let content = std::fs::read_to_string(path).map_err(WorkflowError::FileRead)?;
    let workflow = parse_workflow_content(&content, format)?;

    Ok(workflow)
}

/// Convenience function to execute a workflow from a string path
///
/// This function provides a more ergonomic API for simple use cases.
///
/// # Arguments
///
/// * `path` - Path to the workflow file as a string
///
/// # Returns
///
/// Returns a `WorkflowResult` containing the parsed workflow or an error
///
/// # Examples
///
/// ```rust
/// use colossus::core::engine::execute_workflow_from_path;
///
/// match execute_workflow_from_path("workflow.yml") {
///     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
///     Err(e) => eprintln!("Failed to load workflow: {}", e),
/// }
/// ```
pub fn execute_workflow_from_path<P: AsRef<str>>(path: P) -> WorkflowResult<Workflow> {
    let options = WorkflowOptions::new(path.as_ref());
    execute_workflow(options)
}

/// Convenience function to execute a workflow from a PathBuf
///
/// This function provides a more ergonomic API for simple use cases.
///
/// # Arguments
///
/// * `path` - Path to the workflow file
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
///
/// let path = PathBuf::from("workflow.yml");
/// match execute_workflow_from_pathbuf(path) {
///     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
///     Err(e) => eprintln!("Failed to load workflow: {}", e),
/// }
/// ```
pub fn execute_workflow_from_pathbuf(path: PathBuf) -> WorkflowResult<Workflow> {
    let options = WorkflowOptions::from(path);
    execute_workflow(options)
}
