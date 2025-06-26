//! Workflow output type definition
//!
//! This module contains the `WorkflowOutput` struct that represents
//! the output of a workflow execution.

use serde::{Deserialize, Serialize};
use serde_yml::Value;
use std::collections::HashMap;

/// Represents the output of a workflow execution
///
/// The output is a collection of key-value pairs that represent
/// the results of the workflow execution. Each key is a string
/// identifier, and each value can be any valid YAML value.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::output::WorkflowOutput;
/// use serde_yml::Value;
/// use std::collections::HashMap;
///
/// let mut output_map = HashMap::new();
/// output_map.insert("status".to_string(), Value::String("success".to_string()));
/// output_map.insert("count".to_string(), Value::Number(42.into()));
///
/// let output = WorkflowOutput::from_map(output_map);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOutput(HashMap<String, Value>);

impl WorkflowOutput {
    /// Creates a new empty workflow output
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowOutput` instance with no values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    ///
    /// let output = WorkflowOutput::new();
    /// assert!(output.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a new workflow output from a HashMap
    ///
    /// # Arguments
    ///
    /// * `values` - The HashMap containing the output values
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowOutput` instance with the specified values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    /// use serde_yml::Value;
    /// use std::collections::HashMap;
    ///
    /// let mut values = HashMap::new();
    /// values.insert("result".to_string(), Value::String("success".to_string()));
    ///
    /// let output = WorkflowOutput::from_map(values);
    /// ```
    pub fn from_map(values: HashMap<String, Value>) -> Self {
        Self(values)
    }

    /// Adds a value to the output
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the output value
    /// * `value` - The value to add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    /// use serde_yml::Value;
    ///
    /// let mut output = WorkflowOutput::new();
    /// output.insert("status", Value::String("success".to_string()));
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: Value) {
        self.0.insert(key.into(), value);
    }

    /// Gets a value from the output
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// Returns a reference to the value if it exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    /// use serde_yml::Value;
    ///
    /// let mut output = WorkflowOutput::new();
    /// output.insert("status", Value::String("success".to_string()));
    ///
    /// assert_eq!(output.get("status"), Some(&Value::String("success".to_string())));
    /// assert_eq!(output.get("nonexistent"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Checks if the output is empty
    ///
    /// # Returns
    ///
    /// Returns `true` if the output contains no values, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    ///
    /// let output = WorkflowOutput::new();
    /// assert!(output.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the number of output values
    ///
    /// # Returns
    ///
    /// Returns the number of key-value pairs in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    /// use serde_yml::Value;
    ///
    /// let mut output = WorkflowOutput::new();
    /// output.insert("key1", Value::String("value1".to_string()));
    /// output.insert("key2", Value::String("value2".to_string()));
    ///
    /// assert_eq!(output.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the output values
    ///
    /// # Returns
    ///
    /// Returns an iterator over the key-value pairs in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::output::WorkflowOutput;
    /// use serde_yml::Value;
    ///
    /// let mut output = WorkflowOutput::new();
    /// output.insert("key1", Value::String("value1".to_string()));
    /// output.insert("key2", Value::String("value2".to_string()));
    ///
    /// for (key, value) in output.iter() {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Value> {
        self.0.iter()
    }
}

impl Default for WorkflowOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<String, Value>> for WorkflowOutput {
    fn from(values: HashMap<String, Value>) -> Self {
        Self::from_map(values)
    }
}
