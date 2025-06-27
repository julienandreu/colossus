//! Command-line interface for Colossus
//!
//! This module provides a comprehensive CLI for interacting with the Colossus
//! workflow engine. It supports executing, validating, listing, and inspecting
//! workflows with proper error handling and user-friendly output.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tracing::{debug, error, info, warn};

use crate::core::engine::{ExecuteWorkflowOptions, WorkflowExecutor};
use crate::core::heap::Heap;
use crate::shared::types::workflow::workflow::Workflow;

/// Main CLI configuration structure
///
/// This struct defines the command-line interface using clap. It provides
/// a clean, ergonomic interface for all Colossus operations.
#[derive(Parser)]
#[command(
    name = "colossus",
    about = "Execute workflows with ease",
    version,
    long_about = "Colossus is a workflow execution engine that allows you to define and run workflows using YAML configuration files. It provides a simple, ergonomic interface for executing complex workflows with proper error handling and logging."
)]
pub struct Cli {
    /// Log level for the application
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,

    /// Enable verbose output (overrides log level)
    #[arg(short, long)]
    verbose: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

/// Supported log levels for the application
///
/// This enum provides a type-safe way to specify logging verbosity
/// and maps directly to tracing levels.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LogLevel {
    /// Only show error messages
    Error,
    /// Show warnings and errors
    Warn,
    /// Show informational messages, warnings, and errors
    Info,
    /// Show debug information and all above levels
    Debug,
    /// Show trace information and all above levels
    Trace,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

/// Available CLI commands
///
/// Each variant represents a different operation that can be performed
/// on workflows.
#[derive(Subcommand)]
pub enum Commands {
    /// Execute a workflow from a file
    Execute {
        /// Path to the workflow file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Validate the workflow before execution
        #[arg(short, long)]
        validate: bool,

        /// Output format for results
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },

    /// List available workflows in a directory
    List {
        /// Directory to search for workflows
        #[arg(short, long, default_value = "workflows")]
        path: PathBuf,

        /// Show detailed information for each workflow
        #[arg(short, long)]
        detailed: bool,
    },

    /// Validate a workflow file without executing it
    Validate {
        /// Path to the workflow file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Display information about a workflow
    Info {
        /// Path to the workflow file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

/// Supported output formats for workflow information
///
/// This enum provides different ways to display workflow data,
/// making it easy to integrate with other tools or scripts.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text format
    Text,
    /// JSON format for machine consumption
    Json,
    /// YAML format for configuration files
    Yaml,
}

/// CLI application runner
///
/// This struct provides the main entry point for CLI operations.
/// It handles command parsing, logging setup, and error handling.
pub struct CliApp;

impl CliApp {
    /// Run the CLI application
    ///
    /// This is the main entry point that parses command-line arguments,
    /// sets up logging, and executes the appropriate command.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error with context on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use colossus::application::cli::CliApp;
    ///
    /// // This would be called from main() with proper command line arguments
    /// // if let Err(e) = CliApp::run() {
    /// //     eprintln!("Application error: {}", e);
    /// //     std::process::exit(1);
    /// // }
    /// ```
    pub fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        Self::init_logging(cli.log_level, cli.verbose)?;

        debug!("Starting Colossus CLI application");

        let result = match cli.command {
            Commands::Execute {
                file,
                validate,
                format,
            } => Self::handle_execute(file, validate, format),
            Commands::List {
                path: directory,
                detailed,
            } => Self::handle_list(directory, detailed),
            Commands::Validate { file } => Self::handle_validate(file),
            Commands::Info { file } => Self::handle_info(file),
        };

        match result {
            Ok(()) => {
                info!("Command completed successfully");
                Ok(())
            }
            Err(e) => {
                error!("Command failed: {}", e);
                Err(e)
            }
        }
    }

    /// Initialize logging with the specified level and verbosity
    ///
    /// # Arguments
    ///
    /// * `level` - The base log level for the application
    /// * `verbose` - Whether to enable verbose logging (overrides level)
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if logging setup fails.
    fn init_logging(level: LogLevel, verbose: bool) -> anyhow::Result<()> {
        let level = if verbose {
            tracing::Level::DEBUG
        } else {
            level.into()
        };

        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .init();

        Ok(())
    }

    /// Handle the execute command
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the workflow file
    /// * `validate` - Whether to validate before execution
    /// * `format` - Output format for results
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn handle_execute(file: PathBuf, validate: bool, format: OutputFormat) -> anyhow::Result<()> {
        info!("Executing workflow from file: {:?}", file);

        if validate {
            Self::handle_validate(file.clone())?;
        }

        let mut heap = Heap::new();
        let options = ExecuteWorkflowOptions::new(file);

        match WorkflowExecutor::execute(options, &mut heap) {
            Ok(workflow) => {
                info!("Workflow executed successfully");
                Self::output_workflow(&workflow, format)?;
                Ok(())
            }
            Err(e) => {
                error!("Failed to execute workflow: {}", e);
                Err(anyhow::anyhow!("Workflow execution failed: {}", e))
            }
        }
    }

    /// Handle the list command
    ///
    /// # Arguments
    ///
    /// * `directory` - Directory to search for workflows
    /// * `detailed` - Whether to show detailed information
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn handle_list(directory: PathBuf, detailed: bool) -> anyhow::Result<()> {
        info!("Listing workflows in directory: {:?}", directory);

        if !directory.exists() {
            return Err(anyhow::anyhow!("Directory does not exist: {:?}", directory));
        }

        if !directory.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {:?}", directory));
        }

        let workflows = Self::discover_workflows(&directory)?;

        if workflows.is_empty() {
            println!("No workflow files found in {:?}", directory);
            return Ok(());
        }

        println!("Found {} workflow(s) in {:?}:", workflows.len(), directory);

        for workflow_path in workflows {
            if detailed {
                match Self::get_workflow_info(&workflow_path) {
                    Ok(info) => println!("{}", info),
                    Err(e) => warn!("Failed to get info for {:?}: {}", workflow_path, e),
                }
            } else {
                println!("  {}", workflow_path.file_name().unwrap().to_string_lossy());
            }
        }

        Ok(())
    }

    /// Handle the validate command
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the workflow file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn handle_validate(file: PathBuf) -> anyhow::Result<()> {
        info!("Validating workflow file: {:?}", file);

        if !file.exists() {
            return Err(anyhow::anyhow!("Workflow file does not exist: {:?}", file));
        }

        // Try to parse the workflow to validate it
        let mut heap = Heap::new();
        let options = ExecuteWorkflowOptions::new(file);

        match WorkflowExecutor::execute(options, &mut heap) {
            Ok(workflow) => {
                info!("Workflow validation successful");
                println!("✓ Workflow is valid");
                println!("  Name: {}", workflow.name_or("Unnamed"));
                println!("  Version: {}", workflow.version_or("Unknown"));
                println!("  Nodes: {}", workflow.node_count());
                Ok(())
            }
            Err(e) => {
                error!("Workflow validation failed: {}", e);
                Err(anyhow::anyhow!("Workflow validation failed: {}", e))
            }
        }
    }

    /// Handle the info command
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the workflow file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn handle_info(file: PathBuf) -> anyhow::Result<()> {
        info!("Getting workflow info for file: {:?}", file);

        if !file.exists() {
            return Err(anyhow::anyhow!("Workflow file does not exist: {:?}", file));
        }

        let mut heap = Heap::new();
        let options = ExecuteWorkflowOptions::new(file);

        match WorkflowExecutor::execute(options, &mut heap) {
            Ok(workflow) => {
                Self::output_workflow(&workflow, OutputFormat::Text)?;
                Ok(())
            }
            Err(e) => {
                error!("Failed to get workflow info: {}", e);
                Err(anyhow::anyhow!("Failed to get workflow info: {}", e))
            }
        }
    }

    /// Output workflow information in the specified format
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to output
    /// * `format` - The output format
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn output_workflow(workflow: &Workflow, format: OutputFormat) -> anyhow::Result<()> {
        match format {
            OutputFormat::Text => Self::output_text(workflow),
            OutputFormat::Json => Self::output_json(workflow),
            OutputFormat::Yaml => Self::output_yaml(workflow),
        }
    }

    /// Output workflow information in text format
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to output
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn output_text(workflow: &Workflow) -> anyhow::Result<()> {
        println!("Workflow Information");
        println!("===================");
        println!("Name: {}", workflow.name_or("Unnamed"));
        println!("ID: {}", workflow.id.as_deref().unwrap_or("Not specified"));
        println!("Version: {}", workflow.version_or("Unknown"));
        println!("Nodes: {}", workflow.node_count());

        if let Some(inputs) = &workflow.inputs {
            println!("Inputs: {}", inputs.len());
            for input in inputs {
                println!("  - {}", input.name);
            }
        } else {
            println!("Inputs: None");
        }

        if let Some(variables) = &workflow.variables {
            println!("Variables: {}", variables.len());
            for variable in variables {
                println!("  - {}", variable.name);
            }
        } else {
            println!("Variables: None");
        }

        if let Some(nodes) = &workflow.nodes {
            println!("Node Details:");
            for node in nodes {
                println!("  - {} (type: {})", node.id, node.node_type);
                if let Some(when) = &node.when {
                    println!("    Condition: {}", when);
                }
            }
        }

        Ok(())
    }

    /// Output workflow information in JSON format
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to output
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn output_json(workflow: &Workflow) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(workflow)?;
        println!("{}", json);
        Ok(())
    }

    /// Output workflow information in YAML format
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to output
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error on failure.
    fn output_yaml(workflow: &Workflow) -> anyhow::Result<()> {
        let yaml = serde_yml::to_string(workflow)?;
        println!("{}", yaml);
        Ok(())
    }

    /// Discover workflow files in a directory
    ///
    /// # Arguments
    ///
    /// * `directory` - Directory to search
    ///
    /// # Returns
    ///
    /// Returns a vector of workflow file paths, or an error on failure.
    fn discover_workflows(directory: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let mut workflows = Vec::new();

        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && Self::is_workflow_file(&path) {
                workflows.push(path);
            }
        }

        workflows.sort();
        Ok(workflows)
    }

    /// Check if a file is a workflow file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the file is a workflow file, `false` otherwise.
    fn is_workflow_file(path: &PathBuf) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext.to_lowercase().as_str(), "yml" | "yaml" | "json"))
            .unwrap_or(false)
    }

    /// Get workflow information as a string
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the workflow file
    ///
    /// # Returns
    ///
    /// Returns workflow information as a string, or an error on failure.
    fn get_workflow_info(path: &PathBuf) -> anyhow::Result<String> {
        let mut heap = Heap::new();
        let options = ExecuteWorkflowOptions::new(path);

        let workflow = WorkflowExecutor::execute(options, &mut heap)?;

        Ok(format!(
            "  {} ({} nodes) - {}",
            workflow.name_or("Unnamed"),
            workflow.node_count(),
            path.file_name().unwrap().to_string_lossy()
        ))
    }
}
