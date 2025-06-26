//! Workflow node type definition
//!
//! This module contains the `WorkflowNode` struct that represents
//! a node in a workflow execution graph.

use serde::{Deserialize, Serialize};
use serde_yml::Value;

/// Represents a node in a workflow execution graph
///
/// Each node represents a step in the workflow and defines what action
/// should be performed. Nodes can have conditional execution based on
/// the `when` field.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::node::WorkflowNode;
/// use serde_yml::Value;
///
/// let node = WorkflowNode {
///     id: "log-message".to_string(),
///     node_type: "log".to_string(),
///     input: Value::String("Hello, World!".to_string()),
///     when: Some("debug == true".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    /// Unique identifier for the node
    ///
    /// This ID is used to reference the node throughout the workflow
    /// and must be unique within the workflow.
    pub id: String,

    /// The type of node
    ///
    /// This determines what action the node will perform (e.g., "log",
    /// "http", "script", etc.).
    #[serde(rename = "type")]
    pub node_type: String,

    /// Input data for the node
    ///
    /// This contains the configuration and data that the node needs
    /// to perform its action.
    pub input: Value,

    /// Conditional expression for node execution
    ///
    /// If provided, the node will only execute if this condition
    /// evaluates to true. The condition can reference workflow
    /// variables and inputs.
    pub when: Option<String>,
}

impl WorkflowNode {
    /// Creates a new workflow node
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the node
    /// * `node_type` - The type of node
    /// * `input` - The input data for the node
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowNode` instance without a conditional.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::node::WorkflowNode;
    /// use serde_yml::Value;
    ///
    /// let node = WorkflowNode::new("my-node", "log", Value::String("message".to_string()));
    /// ```
    pub fn new(id: impl Into<String>, node_type: impl Into<String>, input: Value) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            input,
            when: None,
        }
    }

    /// Creates a new workflow node with a conditional
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the node
    /// * `node_type` - The type of node
    /// * `input` - The input data for the node
    /// * `when` - The conditional expression for execution
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowNode` instance with the specified conditional.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::node::WorkflowNode;
    /// use serde_yml::Value;
    ///
    /// let node = WorkflowNode::with_condition(
    ///     "debug-log",
    ///     "log",
    ///     Value::String("debug message".to_string()),
    ///     "debug == true"
    /// );
    /// ```
    pub fn with_condition(
        id: impl Into<String>,
        node_type: impl Into<String>,
        input: Value,
        when: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            input,
            when: Some(when.into()),
        }
    }

    /// Checks if the node has a conditional expression
    ///
    /// # Returns
    ///
    /// Returns `true` if the node has a conditional, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::node::WorkflowNode;
    /// use serde_yml::Value;
    ///
    /// let node = WorkflowNode::new("test", "log", Value::Null);
    /// assert!(!node.has_condition());
    ///
    /// let conditional_node = WorkflowNode::with_condition("test", "log", Value::Null, "true");
    /// assert!(conditional_node.has_condition());
    /// ```
    pub fn has_condition(&self) -> bool {
        self.when.is_some()
    }
}
