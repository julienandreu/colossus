//! Workflow input type definition
//!
//! This module contains the `WorkflowInput` struct that represents
//! an input parameter for a workflow.

use serde::{Deserialize, Serialize};
use serde_yml::Value;

/// Represents an input parameter for a workflow
///
/// Input parameters define the data that must be provided when
/// executing a workflow. They can have default values and type
/// constraints.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::input::WorkflowInput;
/// use serde_yml::Value;
///
/// let input = WorkflowInput {
///     name: "user_id".to_string(),
///     input_type: "string".to_string(),
///     default: Some(Value::String("default_user".to_string())),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInput {
    /// The name of the input parameter
    ///
    /// This name is used to reference the input throughout the workflow.
    pub name: String,

    /// The type of the input parameter
    ///
    /// This defines the expected data type (e.g., "string", "number", "boolean").
    #[serde(rename = "type")]
    pub input_type: String,

    /// The default value for the input parameter
    ///
    /// If provided, this value will be used when no input is specified.
    pub default: Option<Value>,
}

impl WorkflowInput {
    /// Creates a new workflow input
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the input parameter
    /// * `input_type` - The type of the input parameter
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowInput` instance without a default value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::input::WorkflowInput;
    ///
    /// let input = WorkflowInput::new("api_url", "string");
    /// ```
    pub fn new(name: impl Into<String>, input_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_type: input_type.into(),
            default: None,
        }
    }

    /// Creates a new workflow input with a default value
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the input parameter
    /// * `input_type` - The type of the input parameter
    /// * `default` - The default value for the input parameter
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowInput` instance with the specified default value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::input::WorkflowInput;
    /// use serde_yml::Value;
    ///
    /// let input = WorkflowInput::with_default("timeout", "number", Value::Number(30.into()));
    /// ```
    pub fn with_default(
        name: impl Into<String>,
        input_type: impl Into<String>,
        default: Value,
    ) -> Self {
        Self {
            name: name.into(),
            input_type: input_type.into(),
            default: Some(default),
        }
    }
}
