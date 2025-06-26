# Colossus

_Bletchley Park, Winter 1944 — In a dim hall of valves and whirring wheels, a hulking machine cracked ciphers that once held the world in suspense. Colossus was its name, and revelation was its craft._

Today that spirit rises again, honed in Rust and fluent in YAML. This modern Colossus trades paper tape for concurrency, but its purpose is the same: to unravel complexity with relentless precision.

---

⚙️ **Colossus** is an idiomatic, extensible, and ergonomic workflow execution engine written in Rust. It allows you to define, validate, and execute complex workflows using simple YAML configuration files, with a focus on reliability, composability, and developer experience.

---

## Table of Contents

- [Colossus](#colossus)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Installation](#installation)
      - [Prerequisites](#prerequisites)
      - [Build from Source](#build-from-source)
    - [Quick Start](#quick-start)
  - [Usage](#usage)
    - [Defining a Workflow](#defining-a-workflow)
    - [CLI Reference](#cli-reference)
      - [Main Commands](#main-commands)
      - [Global Options](#global-options)
  - [Project Structure](#project-structure)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

---

## Features

- 📝 **YAML-based workflow definitions**
- 🧩 **Composable nodes** for building complex logic
- 🛡️ **Validation** and error reporting
- 🧪 **Testable and documented** with examples
- 🦀 **Idiomatic Rust**: KISS, SOLID, and best practices
- 🛠️ **Extensible**: Add your own node types and logic
- 🧰 **CLI** for workflow execution, validation, and inspection
- 📦 **Minimal dependencies** and fast startup

---

## Getting Started

### Installation

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.87.0 or later)

#### Build from Source

```sh
git clone https://github.com/julienandreu/colossus.git
cd colossus
cargo build --release
```

The binary will be available at `target/release/colossus`.

---

### Quick Start

1. **Write a workflow** (see [Defining a Workflow](#defining-a-workflow))
2. **Run the workflow:**

```sh
colossus execute workflows/simple-log.yml
```

---

## Usage

### Defining a Workflow

Workflows are defined in YAML files. Here is a minimal example:

```yaml
name: Simple Log

nodes:
  - id: HelloWorld
    type: Log
    input: Hello World!

output:
  HelloWorld: ${{ HelloWorld }}
```

- `name`: Human-readable name for the workflow.
- `nodes`: List of steps (nodes) to execute. Each node has an `id`, `type`, and `input`.
- `output`: Defines the output mapping for the workflow.

See the [`workflows/`](workflows/) directory for more examples.

---

### CLI Reference

Run `colossus --help` for the full CLI reference.

#### Main Commands

- **Execute a workflow:**
  ```sh
  colossus execute <FILE> [--validate] [--format <text|json|yaml>]
  ```
- **List available workflows:**
  ```sh
  colossus list [--path <DIR>] [--detailed]
  ```
- **Validate a workflow:**
  ```sh
  colossus validate <FILE>
  ```
- **Show workflow info:**
  ```sh
  colossus info <FILE>
  ```

#### Global Options

- `-l, --log-level <LEVEL>`: Set log level (`error`, `warn`, `info`, `debug`, `trace`)
- `-v, --verbose`: Enable verbose output

---

## Project Structure

```
colossus/
├── src/
│   ├── application/    # CLI and application layer
│   ├── core/           # Workflow engine and core logic
│   ├── infrastructure/ # Infrastructure (logging, config, etc.)
│   ├── nodes/          # Node implementations (extensible)
│   └── shared/         # Shared types and utilities
├── workflows/          # Example workflow files
├── Cargo.toml          # Rust package manifest
├── README.md           # This file
└── LICENSE
```

---

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Ensure all code is documented and tested (`cargo test`)
- Run `cargo fmt` before submitting

---

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.

---

## Acknowledgements

- Inspired by the Rust community and [idiomatic Rust guidelines](https://github.com/mre/idiomatic-rust)
- Built with [clap](https://github.com/clap-rs/clap), [serde](https://serde.rs/), and [tracing](https://github.com/tokio-rs/tracing)

---

*Made with ❤️ by Julien Andreu*
