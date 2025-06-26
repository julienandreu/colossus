//! Workflow variable type definition
//!
//! This module contains the `WorkflowVariable` struct that represents
//! a variable within a workflow.

use serde::{Deserialize, Serialize};
use serde_yml::Value;

/// Represents a variable in a workflow
///
/// Variables provide a way to store and reference data throughout
/// the workflow execution. They can be used to pass data between
/// nodes and provide configuration values.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::variable::WorkflowVariable;
/// use serde_yml::Value;
///
/// let variable = WorkflowVariable {
///     name: "api_key".to_string(),
///     value: Value::String("secret123".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVariable {
    /// The name of the variable
    ///
    /// This name is used to reference the variable throughout the workflow.
    pub name: String,

    /// The value of the variable
    ///
    /// This can be any valid YAML value (string, number, boolean, array, object).
    pub value: Value,
}

impl WorkflowVariable {
    /// Creates a new workflow variable
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable
    /// * `value` - The value of the variable
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowVariable` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::variable::WorkflowVariable;
    /// use serde_yml::Value;
    ///
    /// let variable = WorkflowVariable::new("timeout", Value::Number(30.into()));
    /// ```
    pub fn new(name: impl Into<String>, value: Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}
