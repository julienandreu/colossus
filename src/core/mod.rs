//! Core workflow execution engine
//!
//! This module contains the core business logic for workflow execution,
//! including the engine, heap management, and error handling.
//!
//! # Overview
//!
//! The core module provides the fundamental components needed to execute workflows:
//!
//! - **Engine**: Main workflow execution logic and file parsing
//! - **Heap**: Shared data store for workflow variables and state
//! - **Error Handling**: Comprehensive error types and result handling
//!
//! # Examples
//!
//! ```rust
//! use colossus::core::engine::{ExecuteWorkflowOptions, WorkflowExecutor};
//! use colossus::core::heap::Heap;
//!
//! let mut heap = Heap::new();
//! let options = ExecuteWorkflowOptions::new("workflow.yml");
//! match WorkflowExecutor::execute(options, &mut heap) {
//!     Ok(workflow) => println!("Workflow executed successfully: {:?}", workflow),
//!     Err(e) => eprintln!("Workflow execution failed: {}", e),
//! }
//! ```

pub mod engine;
pub mod heap;
