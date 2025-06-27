use serde_yml::Value;

use crate::core::{engine::WorkflowError, heap::Heap};

/// Options for running a base node
///
/// This struct encapsulates the context and configuration needed
/// to execute a workflow node.
#[derive(Debug, Clone)]
pub struct BaseNodeRunOptions<'a> {
    heap: &'a Heap,
    prefix: String,
}

impl<'a> BaseNodeRunOptions<'a> {
    /// Creates a new `BaseNodeRunOptions` with the specified heap and prefix
    ///
    /// # Arguments
    ///
    /// * `heap` - The heap containing shared data for the workflow execution
    /// * `prefix` - The prefix string for this node's execution context
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::core::heap::Heap;
    /// use colossus::nodes::base::BaseNodeRunOptions;
    ///
    /// let heap = Heap::new();
    /// let options = BaseNodeRunOptions::new(&heap, "node1".to_string());
    /// ```
    pub fn new(heap: &'a Heap, prefix: impl Into<String>) -> Self {
        Self {
            heap,
            prefix: prefix.into(),
        }
    }

    /// Returns a reference to the heap
    ///
    /// # Returns
    ///
    /// Returns a reference to the internal `Heap`.
    pub fn heap(&self) -> &Heap {
        self.heap
    }

    /// Returns a reference to the prefix string
    ///
    /// # Returns
    ///
    /// Returns a reference to the internal prefix string.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Builder method to set the heap
    ///
    /// # Arguments
    ///
    /// * `heap` - New heap to set
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn with_heap(mut self, heap: &'a Heap) -> Self {
        self.heap = heap;
        self
    }

    /// Builder method to set the prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - New prefix to set
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }
}

/// Trait for workflow nodes
///
/// This trait defines the interface that all workflow nodes must implement.
/// It provides a common way to execute different types of nodes with
/// consistent error handling and context.
///
/// # Examples
///
/// ```rust
/// use colossus::nodes::base::{BaseNode, BaseNodeRunOptions};
/// use colossus::core::engine::WorkflowError;
/// use colossus::core::heap::Heap;
/// use serde_yml::Value;
///
/// struct MyNode {
///     input: Option<Value>,
/// }
///
/// impl BaseNode for MyNode {
///     fn execute(&self, _options: BaseNodeRunOptions) -> Result<Value, WorkflowError> {
///         // Node execution logic here
///         Ok(Value::String("Hello, World!".to_string()))
///     }
/// }
/// ```
pub trait BaseNode {
    /// Executes the node with the given options
    ///
    /// # Arguments
    ///
    /// * `options` - The execution options containing heap and context
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the output value or an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::nodes::base::{BaseNode, BaseNodeRunOptions};
    /// use colossus::core::engine::WorkflowError;
    /// use colossus::core::heap::Heap;
    /// use serde_yml::Value;
    ///
    /// struct SimpleNode;
    ///
    /// impl BaseNode for SimpleNode {
    ///     fn execute(&self, options: BaseNodeRunOptions) -> Result<Value, WorkflowError> {
    ///         println!("Executing node with prefix: {}", options.prefix());
    ///         Ok(Value::String("Success".to_string()))
    ///     }
    /// }
    /// ```
    fn execute(&self, options: BaseNodeRunOptions) -> Result<Value, WorkflowError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::heap::Heap;

    #[test]
    fn test_base_node_run_options_new() {
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test_node");

        assert_eq!(options.heap() as *const Heap, &heap as *const Heap);
        assert_eq!(options.prefix(), "test_node");
    }

    #[test]
    fn test_base_node_run_options_with_heap() {
        let heap1 = Heap::new();
        let heap2 = Heap::new();
        let options = BaseNodeRunOptions::new(&heap1, "test_node").with_heap(&heap2);

        assert_eq!(options.heap() as *const Heap, &heap2 as *const Heap);
        assert_eq!(options.prefix(), "test_node");
    }

    #[test]
    fn test_base_node_run_options_with_prefix() {
        let heap = Heap::new();
        let options =
            BaseNodeRunOptions::new(&heap, "initial_prefix").with_prefix("updated_prefix");

        assert_eq!(options.heap() as *const Heap, &heap as *const Heap);
        assert_eq!(options.prefix(), "updated_prefix");
    }

    #[test]
    fn test_base_node_run_options_builder_pattern() {
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "initial")
            .with_heap(&heap)
            .with_prefix("final");

        assert_eq!(options.prefix(), "final");
    }

    #[test]
    fn test_base_node_run_options_string_conversion() {
        let heap = Heap::new();
        let options = BaseNodeRunOptions::new(&heap, "test_node");

        assert_eq!(options.prefix(), "test_node");
    }

    #[test]
    fn test_base_node_run_options_heap_reference() {
        let mut heap = Heap::new();
        heap.insert("test_key", Some(Value::String("test_value".to_string())));

        let options = BaseNodeRunOptions::new(&heap, "test_node");
        let heap_ref = options.heap();

        assert_eq!(
            heap_ref.get("test_key"),
            Some(&Value::String("test_value".to_string()))
        );
    }
}
