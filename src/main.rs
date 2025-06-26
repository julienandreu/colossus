//! Colossus - A workflow execution engine
//!
//! This is the main entry point for the Colossus CLI application.
//! It provides a simple interface for executing workflows defined in YAML configuration files.

use colossus::application::cli::CliApp;

/// Main entry point for the Colossus CLI application.
///
/// This function initializes the CLI application and handles any errors that occur
/// during execution. If an error occurs, it will be printed to stderr and the
/// process will exit with a non-zero status code.
///
/// # Examples
///
/// ```bash
/// # Execute a workflow
/// colossus execute workflow.yml
///
/// # List available workflows
/// colossus list workflows/
///
/// # Validate a workflow
/// colossus validate workflow.yml
/// ```
fn main() {
    // Run the CLI application with proper error handling
    if let Err(e) = CliApp::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
