//! Workflow status type definition
//!
//! This module contains the `Status` enum that represents the various
//! states a workflow can be in during execution.

/// Represents the execution status of a workflow or workflow node
///
/// This enum defines all the possible states that a workflow or individual
/// node can be in during its lifecycle.
///
/// # Examples
///
/// ```rust
/// use colossus::shared::types::workflow::status::Status;
///
/// let status = Status::Running;
/// match status {
///     Status::Running => println!("Workflow is currently running"),
///     Status::Done => println!("Workflow completed successfully"),
///     Status::Failed => println!("Workflow failed"),
///     _ => println!("Workflow is in another state"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Workflow is waiting to be executed
    Pending,

    /// Workflow is preparing to start execution
    Starting,

    /// Workflow is currently being executed
    Running,

    /// Workflow has completed successfully
    Done,

    /// Workflow has failed during execution
    Failed,

    /// Workflow was skipped (e.g., due to conditional logic)
    Skipped,

    /// Workflow status is unknown or undefined
    Unknown,

    /// Workflow execution has been paused
    Paused,

    /// Workflow execution has been stopped
    Stopped,
}

impl Status {
    /// Checks if the status represents a completed state
    ///
    /// # Returns
    ///
    /// Returns `true` if the workflow is done, failed, skipped, or stopped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::status::Status;
    ///
    /// assert!(Status::Done.is_completed());
    /// assert!(Status::Failed.is_completed());
    /// assert!(Status::Skipped.is_completed());
    /// assert!(Status::Stopped.is_completed());
    /// assert!(!Status::Running.is_completed());
    /// ```
    pub fn is_completed(&self) -> bool {
        matches!(
            self,
            Status::Done | Status::Failed | Status::Skipped | Status::Stopped
        )
    }

    /// Checks if the status represents a successful completion
    ///
    /// # Returns
    ///
    /// Returns `true` if the workflow completed successfully.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::status::Status;
    ///
    /// assert!(Status::Done.is_successful());
    /// assert!(!Status::Failed.is_successful());
    /// assert!(!Status::Running.is_successful());
    /// ```
    pub fn is_successful(&self) -> bool {
        matches!(self, Status::Done)
    }

    /// Checks if the status represents a failure
    ///
    /// # Returns
    ///
    /// Returns `true` if the workflow failed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::status::Status;
    ///
    /// assert!(Status::Failed.is_failed());
    /// assert!(!Status::Done.is_failed());
    /// assert!(!Status::Running.is_failed());
    /// ```
    pub fn is_failed(&self) -> bool {
        matches!(self, Status::Failed)
    }

    /// Checks if the status represents an active state
    ///
    /// # Returns
    ///
    /// Returns `true` if the workflow is currently active (starting, running, or paused).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::shared::types::workflow::status::Status;
    ///
    /// assert!(Status::Starting.is_active());
    /// assert!(Status::Running.is_active());
    /// assert!(Status::Paused.is_active());
    /// assert!(!Status::Done.is_active());
    /// assert!(!Status::Failed.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Starting | Status::Running | Status::Paused)
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}
