//! Workflow options type definition
//!
//! This module contains the `WorkflowOptions` struct that represents
//! execution options and configuration for a workflow.

use serde::{Deserialize, Serialize};

/// Represents execution options and configuration for a workflow
///
/// These options control how the workflow should be executed,
/// including performance settings, timeout configurations, and
/// other execution parameters.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::options::WorkflowOptions;
///
/// let options = WorkflowOptions {
///     concurrency: Some(4),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOptions {
    /// Maximum number of concurrent node executions
    ///
    /// This controls how many nodes can be executed simultaneously.
    /// If not specified, the default behavior is determined by the
    /// execution engine.
    pub concurrency: Option<u32>,
}

impl WorkflowOptions {
    /// Creates a new workflow options instance
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowOptions` instance with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::options::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::new();
    /// assert_eq!(options.concurrency, None);
    /// ```
    pub fn new() -> Self {
        Self { concurrency: None }
    }

    /// Creates a new workflow options instance with concurrency limit
    ///
    /// # Arguments
    ///
    /// * `concurrency` - The maximum number of concurrent node executions
    ///
    /// # Returns
    ///
    /// Returns a new `WorkflowOptions` instance with the specified concurrency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::options::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::with_concurrency(4);
    /// assert_eq!(options.concurrency, Some(4));
    /// ```
    pub fn with_concurrency(concurrency: u32) -> Self {
        Self {
            concurrency: Some(concurrency),
        }
    }

    /// Sets the concurrency limit
    ///
    /// # Arguments
    ///
    /// * `concurrency` - The maximum number of concurrent node executions
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::options::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::new()
    ///     .with_concurrency_limit(8);
    /// assert_eq!(options.concurrency, Some(8));
    /// ```
    pub fn with_concurrency_limit(mut self, concurrency: u32) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    /// Gets the concurrency limit or a default value
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to return if concurrency is not set
    ///
    /// # Returns
    ///
    /// Returns the concurrency limit if set, otherwise the default value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::options::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::with_concurrency(4);
    /// assert_eq!(options.concurrency_or(1), 4);
    ///
    /// let options = WorkflowOptions::new();
    /// assert_eq!(options.concurrency_or(1), 1);
    /// ```
    pub fn concurrency_or(&self, default: u32) -> u32 {
        self.concurrency.unwrap_or(default)
    }

    /// Checks if concurrency is limited
    ///
    /// # Returns
    ///
    /// Returns `true` if a concurrency limit is set, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::options::WorkflowOptions;
    ///
    /// let options = WorkflowOptions::with_concurrency(4);
    /// assert!(options.has_concurrency_limit());
    ///
    /// let options = WorkflowOptions::new();
    /// assert!(!options.has_concurrency_limit());
    /// ```
    pub fn has_concurrency_limit(&self) -> bool {
        self.concurrency.is_some()
    }
}

impl Default for WorkflowOptions {
    fn default() -> Self {
        Self::new()
    }
}
