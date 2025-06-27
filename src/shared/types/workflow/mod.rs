//! Workflow-related type definitions
//!
//! This module contains all the type definitions related to workflows,
//! including the main workflow structure and its components.
//!
//! # Overview
//!
//! The workflow module provides comprehensive type definitions for workflow execution:
//!
//! - **Workflow**: Main workflow structure and metadata
//! - **Node**: Individual workflow step definitions
//! - **Input/Output**: Data flow definitions
//! - **Options**: Configuration and execution options
//! - **Status**: Workflow execution status tracking
//! - **Variable**: Workflow variable definitions
//!
//! # Examples
//!
//! ```rust
//! use colossus::shared::types::workflow::{
//!     workflow::Workflow,
//!     node::WorkflowNode,
//!     input::WorkflowInput,
//!     output::WorkflowOutput,
//! };
//!
//! let workflow = Workflow::new("My Workflow")
//!     .with_version("1.0.0");
//! let node = WorkflowNode::new("step1", "Log", None);
//! let input = WorkflowInput::new("message", "string");
//! let output = WorkflowOutput::new();
//! ```

pub mod input;
pub mod node;
pub mod options;
pub mod output;
pub mod status;
pub mod variable;
pub mod workflow;
