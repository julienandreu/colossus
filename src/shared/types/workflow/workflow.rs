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
    /// assert_eq!(workflow.version_or("0.1.0"), "1.0.0");
    ///
    /// let workflow = Workflow {
    ///     version: None,
    ///     ..Default::default()
    /// };
    /// assert_eq!(workflow.version_or("0.1.0"), "0.1.0");
    /// ```
    pub fn version_or(&self, default: &str) -> String {
        self.version.as_deref().unwrap_or(default).to_string()
    }

    /// Returns the number of nodes in the workflow
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
    /// let workflow = Workflow::new("Empty Workflow");
    /// assert_eq!(workflow.node_count(), 0);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.as_ref().map_or(0, |nodes| nodes.len())
    }

    /// Checks if the workflow has any nodes
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
    /// let workflow = Workflow::new("Empty Workflow");
    /// assert!(!workflow.has_nodes());
    /// ```
    pub fn has_nodes(&self) -> bool {
        self.node_count() > 0
    }

    /// Gets a reference to the nodes if they exist
    ///
    /// # Returns
    ///
    /// Returns a reference to the nodes if they exist, `None` otherwise.
    pub fn nodes(&self) -> Option<&[super::node::WorkflowNode]> {
        self.nodes.as_deref()
    }

    /// Gets a mutable reference to the nodes if they exist
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the nodes if they exist, `None` otherwise.
    pub fn nodes_mut(&mut self) -> Option<&mut Vec<super::node::WorkflowNode>> {
        self.nodes.as_mut()
    }

    /// Adds a node to the workflow
    ///
    /// # Arguments
    ///
    /// * `node` - The node to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::{workflow::Workflow, node::WorkflowNode};
    /// use serde_yml::Value;
    ///
    /// let mut workflow = Workflow::new("My Workflow");
    /// let node = WorkflowNode::new("test", "log", Some(Value::String("message".to_string())));
    /// workflow.add_node(node);
    /// assert_eq!(workflow.node_count(), 1);
    /// ```
    pub fn add_node(&mut self, node: super::node::WorkflowNode) {
        if let Some(nodes) = &mut self.nodes {
            nodes.push(node);
        } else {
            self.nodes = Some(vec![node]);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::types::workflow::node::WorkflowNode;
    use serde_yml::Value;

    #[test]
    fn test_workflow_new() {
        let workflow = Workflow::new("Test Workflow");
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.id, None);
        assert_eq!(workflow.version, None);
    }

    #[test]
    fn test_workflow_with_id() {
        let workflow = Workflow::new("Test Workflow").with_id("workflow-123");
        assert_eq!(workflow.id, Some("workflow-123".to_string()));
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
    }

    #[test]
    fn test_workflow_with_version() {
        let workflow = Workflow::new("Test Workflow").with_version("1.0.0");
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
    }

    #[test]
    fn test_workflow_name_or() {
        let workflow = Workflow::new("Test Workflow");
        assert_eq!(workflow.name_or("Default"), "Test Workflow");

        let workflow = Workflow {
            name: None,
            ..Default::default()
        };
        assert_eq!(workflow.name_or("Default"), "Default");
    }

    #[test]
    fn test_workflow_version_or() {
        let workflow = Workflow::new("Test Workflow").with_version("1.0.0");
        assert_eq!(workflow.version_or("0.1.0"), "1.0.0");

        let workflow = Workflow {
            version: None,
            ..Default::default()
        };
        assert_eq!(workflow.version_or("0.1.0"), "0.1.0");
    }

    #[test]
    fn test_workflow_node_count() {
        let workflow = Workflow::new("Empty Workflow");
        assert_eq!(workflow.node_count(), 0);

        let mut workflow = Workflow::new("Workflow with Nodes");
        let node1 = WorkflowNode::new("node1", "Log", None);
        let node2 = WorkflowNode::new("node2", "Log", None);
        workflow.add_node(node1);
        workflow.add_node(node2);
        assert_eq!(workflow.node_count(), 2);
    }

    #[test]
    fn test_workflow_has_nodes() {
        let workflow = Workflow::new("Empty Workflow");
        assert!(!workflow.has_nodes());

        let mut workflow = Workflow::new("Workflow with Nodes");
        let node = WorkflowNode::new("node1", "Log", None);
        workflow.add_node(node);
        assert!(workflow.has_nodes());
    }

    #[test]
    fn test_workflow_nodes() {
        let workflow = Workflow::new("Empty Workflow");
        assert!(workflow.nodes().is_none());

        let mut workflow = Workflow::new("Workflow with Nodes");
        let node = WorkflowNode::new("node1", "Log", None);
        workflow.add_node(node);

        let nodes = workflow.nodes().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "node1");
    }

    #[test]
    fn test_workflow_nodes_mut() {
        let mut workflow = Workflow::new("Workflow with Nodes");
        let node = WorkflowNode::new("node1", "Log", None);
        workflow.add_node(node);

        let nodes_mut = workflow.nodes_mut().unwrap();
        assert_eq!(nodes_mut.len(), 1);
        nodes_mut[0].id = "updated_node".to_string();

        let nodes = workflow.nodes().unwrap();
        assert_eq!(nodes[0].id, "updated_node");
    }

    #[test]
    fn test_workflow_add_node() {
        let mut workflow = Workflow::new("Test Workflow");
        assert_eq!(workflow.node_count(), 0);

        let node1 = WorkflowNode::new("node1", "Log", Some(Value::String("message1".to_string())));
        workflow.add_node(node1);
        assert_eq!(workflow.node_count(), 1);

        let node2 = WorkflowNode::new("node2", "Log", Some(Value::String("message2".to_string())));
        workflow.add_node(node2);
        assert_eq!(workflow.node_count(), 2);

        let nodes = workflow.nodes().unwrap();
        assert_eq!(nodes[0].id, "node1");
        assert_eq!(nodes[1].id, "node2");
    }

    #[test]
    fn test_workflow_default() {
        let workflow = Workflow::default();
        assert_eq!(workflow.id, None);
        assert_eq!(workflow.name, None);
        assert_eq!(workflow.version, None);
        assert!(workflow.variables.is_none());
        assert!(workflow.inputs.is_none());
        assert!(workflow.nodes.is_none());
        assert!(workflow.output.is_none());
        assert!(workflow.options.is_none());
    }

    #[test]
    fn test_workflow_builder_pattern() {
        let workflow = Workflow::new("Test Workflow")
            .with_id("workflow-123")
            .with_version("1.0.0");

        assert_eq!(workflow.name, Some("Test Workflow".to_string()));
        assert_eq!(workflow.id, Some("workflow-123".to_string()));
        assert_eq!(workflow.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_workflow_string_conversion() {
        let workflow = Workflow::new("Test Workflow");
        assert_eq!(workflow.name, Some("Test Workflow".to_string()));

        let workflow = Workflow::new(String::from("Dynamic Workflow"));
        assert_eq!(workflow.name, Some("Dynamic Workflow".to_string()));
    }
}
