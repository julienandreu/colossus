//! Main workflow type definition
//!
//! This module contains the core `Workflow` struct that represents
//! a complete workflow definition with all its components.

use serde::{Deserialize, Serialize};

/// Represents a complete workflow definition
///
/// This struct encapsulates all the information needed to define and execute
/// a workflow, including metadata, inputs, nodes, outputs, and configuration.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::workflow::Workflow;
///
/// let workflow = Workflow {
///     id: Some("my-workflow".to_string()),
///     name: Some("My Workflow".to_string()),
///     version: Some("1.0.0".to_string()),
///     variables: None,
///     inputs: None,
///     nodes: None,
///     output: None,
///     options: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique identifier for the workflow
    ///
    /// This field is optional but recommended for workflows that will be
    /// stored or referenced by other systems.
    pub id: Option<String>,

    /// Human-readable name for the workflow
    ///
    /// This field provides a descriptive name that can be displayed
    /// in user interfaces and documentation.
    pub name: Option<String>,

    /// Version of the workflow definition
    ///
    /// This field should follow semantic versioning (e.g., "1.0.0") to
    /// track changes and ensure compatibility.
    pub version: Option<String>,

    /// Global variables available throughout the workflow
    ///
    /// These variables can be referenced by nodes and provide a way to
    /// share data across the entire workflow execution.
    pub variables: Option<Vec<super::variable::WorkflowVariable>>,

    /// Input parameters for the workflow
    ///
    /// These define the expected inputs that must be provided when
    /// executing the workflow.
    pub inputs: Option<Vec<super::input::WorkflowInput>>,

    /// Nodes that make up the workflow execution graph
    ///
    /// Each node represents a step in the workflow and defines what
    /// action should be performed.
    pub nodes: Option<Vec<super::node::WorkflowNode>>,

    /// Output definition for the workflow
    ///
    /// This defines what data the workflow will produce as its result.
    pub output: Option<super::output::WorkflowOutput>,

    /// Execution options and configuration
    ///
    /// These options control how the workflow should be executed,
    /// including timeout settings, retry policies, etc.
    pub options: Option<super::options::WorkflowOptions>,
}

impl Workflow {
    /// Creates a new workflow with the given name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the workflow
    ///
    /// # Returns
    ///
    /// Returns a new `Workflow` instance with the specified name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow");
    /// assert_eq!(workflow.name, Some("My Workflow".to_string()));
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: None,
            name: Some(name.into()),
            version: None,
            variables: None,
            inputs: None,
            nodes: None,
            output: None,
            options: None,
        }
    }

    /// Sets the workflow ID
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the workflow
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow")
    ///     .with_id("workflow-123");
    /// assert_eq!(workflow.id, Some("workflow-123".to_string()));
    /// ```
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the workflow version
    ///
    /// # Arguments
    ///
    /// * `version` - The version string (should follow semantic versioning)
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow")
    ///     .with_version("1.0.0");
    /// assert_eq!(workflow.version, Some("1.0.0".to_string()));
    /// ```
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Gets the workflow name or a default value
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to return if name is None
    ///
    /// # Returns
    ///
    /// Returns the workflow name if present, otherwise the default value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow");
    /// assert_eq!(workflow.name_or("Unnamed"), "My Workflow");
    ///
    /// let workflow = Workflow {
    ///     name: None,
    ///     ..Default::default()
    /// };
    /// assert_eq!(workflow.name_or("Unnamed"), "Unnamed");
    /// ```
    pub fn name_or(&self, default: &str) -> String {
        self.name.as_deref().unwrap_or(default).to_string()
    }

    /// Gets the workflow version or a default value
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to return if version is None
    ///
    /// # Returns
    ///
    /// Returns the workflow version if present, otherwise the default value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow").with_version("1.0.0");
    /// assert_eq!(workflow.version_or("Unknown"), "1.0.0");
    ///
    /// let workflow = Workflow::new("My Workflow");
    /// assert_eq!(workflow.version_or("Unknown"), "Unknown");
    /// ```
    pub fn version_or(&self, default: &str) -> String {
        self.version.as_deref().unwrap_or(default).to_string()
    }

    /// Gets the number of nodes in the workflow
    ///
    /// # Returns
    ///
    /// Returns the number of nodes, or 0 if no nodes are defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow");
    /// assert_eq!(workflow.node_count(), 0);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.as_ref().map(|nodes| nodes.len()).unwrap_or(0)
    }

    /// Checks if the workflow has any nodes defined
    ///
    /// # Returns
    ///
    /// Returns `true` if the workflow has nodes, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::workflow::Workflow;
    ///
    /// let workflow = Workflow::new("My Workflow");
    /// assert!(!workflow.has_nodes());
    /// ```
    pub fn has_nodes(&self) -> bool {
        self.node_count() > 0
    }
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            version: None,
            variables: None,
            inputs: None,
            nodes: None,
            output: None,
            options: None,
        }
    }
}
