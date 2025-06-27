//! Application layer for Colossus
//!
//! This module contains the application logic and user interface components,
//! including the CLI application and any future GUI or web interfaces.
//!
//! # Overview
//!
//! The application module provides user-facing functionality:
//!
//! - **CLI**: Command-line interface for workflow execution
//! - **User Interface**: Future GUI and web interface components
//! - **Application Logic**: High-level application coordination
//!
//! # Examples
//!
//! ```rust
//! use colossus::application::cli::CliApp;
//!
//! // The CLI application is typically run from main() with command line arguments
//! // if let Err(e) = CliApp::run() {
//! //     eprintln!("Application error: {}", e);
//! //     std::process::exit(1);
//! // }
//! ```

pub mod cli;
