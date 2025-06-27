# Colossus

A robust and ergonomic workflow execution engine for Rust that allows you to define and run workflows using YAML configuration files.

## Features

- **Simple YAML/JSON Configuration**: Define workflows using human-readable YAML or JSON files
- **Extensible Node System**: Easy to add new node types for different operations
- **Variable Substitution**: Use `${{ variable_name }}` syntax for dynamic values
- **Comprehensive Error Handling**: Detailed error messages and proper error chaining
- **CLI Interface**: Full-featured command-line interface for workflow management
- **Logging Integration**: Built-in logging with configurable levels
- **Rust Idiomatic**: Follows Rust best practices and conventions
- **Type Safety**: Strongly typed workflow definitions with validation

## Installation

### From Source

```bash
git clone https://github.com/julienandreu/colossus.git
cd colossus
cargo build --release
```

### From Cargo

```bash
cargo install colossus
```

## Quick Start

### 1. Create a Workflow

Create a file named `simple-log.yml`:

```yaml
name: "Simple Log Workflow"
version: "1.0.0"
nodes:
  - id: "Hello"
    type: "Log"
    input: "Hello"
  
  - id: "World"
    type: "Log"
    input: "World"

  - id: "HelloWorld"
    type: "Log"
    input: "${{ Hello }} ${{ World }}!"

output:
  Hello: "${{ Hello }}"
  World: "${{ World }}"
  HelloWorld: "${{ HelloWorld }}"
```

### 2. Execute the Workflow

```bash
colossus execute simple-log.yml
```

### 3. View Workflow Information

```bash
colossus info simple-log.yml
```

## CLI Usage

### Execute a Workflow

```bash
# Basic execution
colossus execute workflow.yml

# With validation
colossus execute workflow.yml --validate

# Output in different formats
colossus execute workflow.yml --format json
colossus execute workflow.yml --format yaml
```

### List Workflows

```bash
# List workflows in current directory
colossus list

# List workflows in specific directory
colossus list --path /path/to/workflows

# Show detailed information
colossus list --detailed
```

### Validate a Workflow

```bash
colossus validate workflow.yml
```

### Get Workflow Information

```bash
colossus info workflow.yml
```

### Logging

```bash
# Set log level
colossus --log-level debug execute workflow.yml

# Enable verbose output
colossus --verbose execute workflow.yml
```

## Workflow Configuration

### Basic Structure

```yaml
name: "My Workflow"
version: "1.0.0"
id: "unique-workflow-id"

variables:
  - name: "environment"
    value: "production"

inputs:
  - name: "user_name"
    type: "string"
    required: true

nodes:
  - id: "log_message"
    type: "Log"
    input: "Hello ${{ user_name }}!"

output:
  type: "object"
  properties:
    result:
      type: "string"
```

### Variable Substitution

Use `${{ variable_name }}` syntax to reference variables:

```yaml
nodes:
  - id: "welcome"
    type: "Log"
    input: "Welcome to ${{ environment }} environment!"
```

### Conditional Execution

Use the `when` field for conditional node execution:

```yaml
nodes:
  - id: "debug_log"
    type: "Log"
    input: "Debug information"
    when: "debug == true"
```

## Architecture

### Core Components

- **Engine**: Main workflow execution logic with file format detection
- **Heap**: Shared data store for workflow variables and state management
- **Nodes**: Individual workflow steps with extensible node system
- **CLI**: Command-line interface with comprehensive command support
- **Types**: Strongly typed workflow data structures

### Design Principles

- **SOLID**: Single responsibility, open/closed, dependency inversion
- **KISS**: Keep it simple and straightforward
- **Rust Idiomatic**: Follow Rust conventions and best practices
- **Type Safety**: Strong typing throughout the codebase

## Extending Colossus

### Adding New Node Types

1. Create a new node module in `src/nodes/`
2. Implement the `BaseNode` trait
3. Register the node in `NodeBuilder::build()`

Example:

```rust
use crate::nodes::base::{BaseNode, BaseNodeRunOptions};
use crate::core::engine::WorkflowError;
use serde_yml::Value;

pub struct HttpNode {
    input: Option<Value>,
}

impl BaseNode for HttpNode {
    fn execute(&self, _options: BaseNodeRunOptions) -> Result<Value, WorkflowError> {
        // HTTP request logic here
        Ok(Value::String("HTTP response".to_string()))
    }
}
```

### Custom Error Types

Use `thiserror` for custom error types:

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyNodeError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- execute workflow.yml
```

### Project Structure

```
src/
├── application/     # CLI application logic
│   └── cli/        # Command-line interface
├── core/           # Core workflow engine
│   ├── engine.rs   # Workflow execution engine
│   └── heap.rs     # Shared data store
├── infrastructure/ # Infrastructure concerns
├── nodes/          # Workflow node implementations
│   ├── base/       # Base node trait and types
│   └── log/        # Logging node implementation
├── shared/         # Shared types and utilities
│   └── types/      # Workflow type definitions
├── lib.rs          # Library entry point
└── main.rs         # Binary entry point
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_workflow_execution

# Run with output
cargo test -- --nocapture
```

## Current Node Types

### Log Node

The Log node outputs messages to the console and is useful for debugging and providing feedback during workflow execution.

```yaml
nodes:
  - id: "debug_info"
    type: "Log"
    input: "Processing step completed"
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Code Style

- Follow Rust formatting guidelines (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Write comprehensive documentation
- Include examples in doc comments

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.2.0
- Improved architecture with better separation of concerns
- Enhanced error handling and logging
- More Rust idiomatic code
- Better CLI interface with comprehensive commands
- Strongly typed workflow definitions
- Comprehensive documentation and examples
- Variable substitution support
- Multiple output formats (JSON, YAML, Text)
- Workflow validation and information display

### v0.1.0
- Initial release
- Basic workflow execution
- YAML/JSON support
- Simple CLI interface
