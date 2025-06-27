//! Shared types and utilities
//!
//! This module contains types and utilities that are shared across
//! different parts of the application, including workflow definitions
//! and common data structures.
//!
//! # Overview
//!
//! The shared module provides common types and utilities used throughout the application:
//!
//! - **Types**: Core type definitions for workflows, nodes, inputs, outputs, etc.
//! - **Utilities**: Common functionality shared across modules
//!
//! # Examples
//!
//! ```rust
//! use colossus::shared::types::workflow::workflow::Workflow;
//! use colossus::shared::types::workflow::node::WorkflowNode;
//!
//! let workflow = Workflow::new("My Workflow");
//! let node = WorkflowNode::new("node1", "Log", None);
//! ```

pub mod types;
