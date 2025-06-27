//! Workflow node implementations
//!
//! This module contains the implementations of different workflow nodes
//! and the node builder system for creating and configuring nodes.
//!
//! # Overview
//!
//! The nodes module provides the building blocks for workflow execution:
//!
//! - **Base**: Core node trait and execution options
//! - **Log**: Logging node for debugging and output
//! - **Builder**: Fluent interface for creating nodes
//!
//! # Examples
//!
//! ```rust
//! use colossus::nodes::{NodeBuilder, base::BaseNode};
//! use colossus::shared::types::workflow::node::WorkflowNode;
//! use colossus::core::heap::Heap;
//! use serde_yml::Value;
//!
//! let mut heap = Heap::new();
//! let node_config = WorkflowNode::new("log1", "Log", Some(Value::String("Hello".to_string())));
//! let node = NodeBuilder::new()
//!     .with_workflow_node(node_config)
//!     .build(&mut heap)
//!     .expect("Failed to build node");
//! ```

use crate::{
    core::{engine::WorkflowResult, heap::Heap},
    nodes::{base::BaseNode, log::LogNode},
};
use serde_yml::Value;

use crate::shared::types::workflow::node::WorkflowNode;

pub mod base;
pub mod log;

/// Builder for creating workflow nodes
///
/// This struct provides a fluent interface for building nodes with
/// proper configuration and error handling.
#[derive(Debug, Clone)]
pub struct NodeBuilder {
    workflow_node: Option<WorkflowNode>,
    input: Option<Value>,
}

impl NodeBuilder {
    /// Creates a new node builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::NodeBuilder;
    ///
    /// let builder = NodeBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            workflow_node: None,
            input: None,
        }
    }

    /// Sets the workflow node configuration
    ///
    /// # Arguments
    ///
    /// * `workflow_node` - The workflow node configuration
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::NodeBuilder;
    /// use colossus::shared::types::workflow::node::WorkflowNode;
    /// use serde_yml::Value;
    ///
    /// let node = WorkflowNode::new("test", "log", Some(Value::String("message".to_string())));
    /// let builder = NodeBuilder::new().with_workflow_node(node);
    /// ```
    pub fn with_workflow_node(mut self, workflow_node: WorkflowNode) -> Self {
        self.input = workflow_node.input.clone();
        self.workflow_node = Some(workflow_node);
        self
    }

    /// Sets the input value for the node
    ///
    /// # Arguments
    ///
    /// * `input` - The input value for the node
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::NodeBuilder;
    /// use serde_yml::Value;
    ///
    /// let builder = NodeBuilder::new().with_input(Value::String("message".to_string()));
    /// ```
    pub fn with_input(mut self, input: Value) -> Self {
        self.input = Some(input);
        self
    }

    /// Builds a node instance from the configuration
    ///
    /// # Arguments
    ///
    /// * `heap` - The heap containing shared data
    ///
    /// # Returns
    ///
    /// Returns a `WorkflowResult` containing the built node or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::NodeBuilder;
    /// use colossus::core::heap::Heap;
    /// use colossus::shared::types::workflow::node::WorkflowNode;
    /// use serde_yml::Value;
    ///
    /// let mut heap = Heap::new();
    /// let node = WorkflowNode::new("test", "Log", Some(Value::String("message".to_string())));
    /// let builder = NodeBuilder::new().with_workflow_node(node);
    /// let node_instance = builder.build(&mut heap).expect("Failed to build node");
    /// ```
    pub fn build(self, heap: &mut Heap) -> WorkflowResult<Box<dyn BaseNode>> {
        let input = heap.parse(self.input);

        let node_type = self
            .workflow_node
            .as_ref()
            .ok_or_else(|| {
                crate::core::engine::WorkflowError::NodeBuilder(
                    "No workflow node configuration provided".to_string(),
                )
            })?
            .node_type
            .clone();

        match node_type.as_str() {
            "Log" => Ok(Box::new(LogNode::new(input))),
            _ => Err(crate::core::engine::WorkflowError::InvalidNode(node_type)),
        }
    }

    /// Gets a reference to the workflow node configuration
    ///
    /// # Returns
    ///
    /// Returns a reference to the workflow node if set, `None` otherwise.
    pub fn workflow_node(&self) -> Option<&WorkflowNode> {
        self.workflow_node.as_ref()
    }

    /// Gets a reference to the input value
    ///
    /// # Returns
    ///
    /// Returns a reference to the input value if set, `None` otherwise.
    pub fn input(&self) -> Option<&Value> {
        self.input.as_ref()
    }
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<WorkflowNode> for NodeBuilder {
    fn from(workflow_node: WorkflowNode) -> Self {
        Self::new().with_workflow_node(workflow_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::types::workflow::node::WorkflowNode;

    #[test]
    fn test_node_builder_new() {
        let builder = NodeBuilder::new();
        assert!(builder.workflow_node().is_none());
        assert!(builder.input().is_none());
    }

    #[test]
    fn test_node_builder_default() {
        let builder = NodeBuilder::default();
        assert!(builder.workflow_node().is_none());
        assert!(builder.input().is_none());
    }

    #[test]
    fn test_node_builder_with_workflow_node() {
        let node = WorkflowNode::new("test", "Log", Some(Value::String("message".to_string())));
        let builder = NodeBuilder::new().with_workflow_node(node.clone());

        let builder_node = builder.workflow_node().unwrap();
        assert_eq!(builder_node.id, node.id);
        assert_eq!(builder.input(), Some(&Value::String("message".to_string())));
    }

    #[test]
    fn test_node_builder_with_input() {
        let input = Value::String("custom input".to_string());
        let builder = NodeBuilder::new().with_input(input.clone());

        assert_eq!(builder.input(), Some(&input));
    }

    #[test]
    fn test_node_builder_build_log_node() {
        let mut heap = Heap::new();
        let node = WorkflowNode::new(
            "log1",
            "Log",
            Some(Value::String("Hello, World!".to_string())),
        );
        let builder = NodeBuilder::new().with_workflow_node(node);

        let result = builder.build(&mut heap);
        assert!(result.is_ok());

        let _node_instance = result.unwrap();
        // Note: We can't easily test the trait object without more complex setup
    }

    #[test]
    fn test_node_builder_build_invalid_node() {
        let mut heap = Heap::new();
        let node = WorkflowNode::new("invalid", "InvalidNode", None);
        let builder = NodeBuilder::new().with_workflow_node(node);

        let result = builder.build(&mut heap);
        assert!(result.is_err());

        if let Err(crate::core::engine::WorkflowError::InvalidNode(node_type)) = result {
            assert_eq!(node_type, "InvalidNode");
        } else {
            panic!("Expected InvalidNode error");
        }
    }

    #[test]
    fn test_node_builder_build_no_workflow_node() {
        let mut heap = Heap::new();
        let builder = NodeBuilder::new();

        let result = builder.build(&mut heap);
        assert!(result.is_err());

        if let Err(crate::core::engine::WorkflowError::NodeBuilder(msg)) = result {
            assert!(msg.contains("No workflow node configuration provided"));
        } else {
            panic!("Expected NodeBuilder error");
        }
    }

    #[test]
    fn test_node_builder_from_workflow_node() {
        let node = WorkflowNode::new("test", "Log", Some(Value::String("message".to_string())));
        let builder = NodeBuilder::from(node.clone());

        let builder_node = builder.workflow_node().unwrap();
        assert_eq!(builder_node.id, node.id);
        assert_eq!(builder.input(), Some(&Value::String("message".to_string())));
    }

    #[test]
    fn test_node_builder_with_heap_parsing() {
        let mut heap = Heap::new();
        heap.insert("name", Some(Value::String("John".to_string())));

        let node = WorkflowNode::new(
            "log1",
            "Log",
            Some(Value::String("Hello {{name}}".to_string())),
        );
        let builder = NodeBuilder::new().with_workflow_node(node);

        let result = builder.build(&mut heap);
        assert!(result.is_ok());

        let _node_instance = result.unwrap();
        // Note: We can't easily test the trait object without more complex setup
    }
}
