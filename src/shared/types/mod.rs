//! Type definitions for the application
//!
//! This module contains all the type definitions used throughout the application,
//! organized by domain and functionality.
//!
//! # Overview
//!
//! The types module provides structured type definitions for the application:
//!
//! - **Workflow**: Complete workflow definitions and components
//! - **Domain Types**: Specialized types for different application domains
//!
//! # Examples
//!
//! ```rust
//! use colossus::shared::types::workflow::workflow::Workflow;
//! use colossus::shared::types::workflow::node::WorkflowNode;
//! use colossus::shared::types::workflow::input::WorkflowInput;
//!
//! let workflow = Workflow::new("Example Workflow");
//! let node = WorkflowNode::new("step1", "Log", None);
//! let input = WorkflowInput::new("param1", "string");
//! ```

pub mod workflow;
