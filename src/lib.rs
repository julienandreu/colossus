//! Colossus - A workflow execution engine
//!
//! This library provides a robust and ergonomic interface for defining and executing
//! workflows using YAML configuration files. It follows idiomatic Rust practices
//! and provides comprehensive error handling.
//!
//! # Architecture
//!
//! The library is organized into several modules:
//!
//! - `application`: CLI application logic and user interface
//! - `core`: Core workflow execution engine
//! - `shared`: Shared types and utilities
//! - `nodes`: Workflow node implementations
//! - `infrastructure`: Infrastructure concerns (logging, configuration, etc.)
//!
//! # Examples
//!
//! ```rust
//! use colossus::core::engine::{WorkflowOptions, execute_workflow};
//!
//! let options = WorkflowOptions::new("workflow.yml");
//! match execute_workflow(options) {
//!     Ok(workflow) => println!("Workflow loaded: {:?}", workflow),
//!     Err(e) => eprintln!("Failed to execute workflow: {}", e),
//! }
//! ```

pub mod application;
pub mod core;
pub mod infrastructure;
pub mod nodes;
pub mod shared;
