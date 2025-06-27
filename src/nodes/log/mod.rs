use serde_yml::Value;
use tracing::info;

use crate::{core::engine::WorkflowError, nodes::base::BaseNode};

/// A node that logs messages to the console
///
/// This node is useful for debugging and providing feedback during
/// workflow execution.
#[derive(Debug, Clone)]
pub struct LogNode {
    input: Option<Value>,
}

impl LogNode {
    /// Creates a new log node with the specified input
    ///
    /// # Arguments
    ///
    /// * `input` - The input value to log
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::log::LogNode;
    /// use serde_yml::Value;
    ///
    /// let node = LogNode::new(Some(Value::String("Hello, World!".to_string())));
    /// ```
    pub fn new(input: Option<Value>) -> Self {
        Self { input }
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

impl BaseNode for LogNode {
    fn execute(&self, _options: super::base::BaseNodeRunOptions) -> Result<Value, WorkflowError> {
        match &self.input {
            Some(value) => {
                info!("Log node output: {:?}", value);
                Ok(value.clone())
            }
            None => {
                let error_msg = "Log node requires an input value".to_string();
                info!("Log node error: {}", error_msg);
                Err(WorkflowError::NodeExecutionFailed(error_msg))
            }
        }
    }
}

impl Default for LogNode {
    fn default() -> Self {
        Self { input: None }
    }
}

impl From<Value> for LogNode {
    fn from(input: Value) -> Self {
        Self::new(Some(input))
    }
}

impl From<Option<Value>> for LogNode {
    fn from(input: Option<Value>) -> Self {
        Self::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::heap::Heap;
    use crate::nodes::base::BaseNodeRunOptions;

    #[test]
    fn test_log_node_new() {
        let input = Value::String("test message".to_string());
        let node = LogNode::new(Some(input.clone()));
        assert_eq!(node.input(), Some(&input));
    }

    #[test]
    fn test_log_node_new_none() {
        let node = LogNode::new(None);
        assert_eq!(node.input(), None);
    }

    #[test]
    fn test_log_node_default() {
        let node = LogNode::default();
        assert_eq!(node.input(), None);
    }

    #[test]
    fn test_log_node_from_value() {
        let input = Value::String("test message".to_string());
        let node = LogNode::from(input.clone());
        assert_eq!(node.input(), Some(&input));
    }

    #[test]
    fn test_log_node_from_option_value() {
        let input = Value::String("test message".to_string());
        let node = LogNode::from(Some(input.clone()));
        assert_eq!(node.input(), Some(&input));
    }

    #[test]
    fn test_log_node_from_option_none() {
        let node = LogNode::from(None as Option<Value>);
        assert_eq!(node.input(), None);
    }

    #[test]
    fn test_log_node_execute_with_input() {
        let input = Value::String("Hello, World!".to_string());
        let node = LogNode::new(Some(input.clone()));
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test".to_string());

        let result = node.execute(options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_log_node_execute_without_input() {
        let node = LogNode::new(None);
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test".to_string());

        let result = node.execute(options);
        assert!(result.is_err());

        match result.unwrap_err() {
            crate::core::engine::WorkflowError::NodeExecutionFailed(msg) => {
                assert!(msg.contains("Log node requires an input value"));
            }
            _ => panic!("Expected NodeExecutionFailed error"),
        }
    }

    #[test]
    fn test_log_node_execute_with_number_input() {
        let input = Value::Number(42.into());
        let node = LogNode::new(Some(input.clone()));
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test".to_string());

        let result = node.execute(options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_log_node_execute_with_boolean_input() {
        let input = Value::Bool(true);
        let node = LogNode::new(Some(input.clone()));
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test".to_string());

        let result = node.execute(options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_log_node_execute_with_null_input() {
        let input = Value::Null;
        let node = LogNode::new(Some(input.clone()));
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test".to_string());

        let result = node.execute(options);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }
}
