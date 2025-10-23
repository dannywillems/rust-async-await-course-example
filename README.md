# rust-async-await-course-example

Courses on Rust async/await with examples demonstrating how async functions are transformed into state machines, multiple await points, and variable scoping patterns.

## Overview

This repository provides practical examples of Rust's async/await functionality, showing how the compiler transforms async functions into state machines and how to effectively work with asynchronous code.

## Features

- **State Machine Examples**: See how async functions are transformed into state machines
- **Multiple Await Points**: Understanding how functions suspend and resume at each await
- **Variable Scoping**: Learn how variables are handled across await boundaries
- **Real-world Patterns**: HTTP requests and concurrent execution examples
- **Comprehensive Tests**: Unit tests for all async patterns
- **Full CI/CD**: Automated formatting, linting, testing, and building

## Prerequisites

- Rust 1.75 or later (2021 edition)
- Cargo (comes with Rust)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/dannywillems/rust-async-await-course-example.git
cd rust-async-await-course-example

# Run the examples
make run

# Or using cargo directly
cargo run
```

## Dependencies

- **tokio**: Async runtime with full features
- **reqwest**: HTTP client for async requests

## Makefile Targets

This project includes a comprehensive Makefile with the following targets:

### Development Targets

- **`make help`**: Display all available targets with descriptions
- **`make build`**: Build the project in release mode
- **`make run`**: Run the project and execute all examples
- **`make clean`**: Remove all build artifacts

### Code Quality Targets

- **`make format`**: Format all code using rustfmt
- **`make format-check`**: Check if code is properly formatted (CI-friendly)
- **`make lint`** or **`make clippy`**: Run clippy linter with strict warnings
- **`make test`**: Run all unit tests

### Inspection Targets

- **`make expand`**: Show macro expansions (installs cargo-expand if needed)
- **`make mir`**: Display MIR (Mid-level Intermediate Representation) - requires nightly Rust
- **`make inspect`**: Inspect async transformations and state machines

### CI Target

- **`make ci`**: Run all CI checks (format-check, lint, test, build) - this is what runs in GitHub Actions

### Examples

```bash
# Format your code
make format

# Check if code passes all CI checks before committing
make ci

# Inspect how async functions are transformed
make inspect

# Run tests
make test
```

## Project Structure

```
.
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI pipeline
├── src/
│   ├── lib.rs               # Library with async function examples
│   └── main.rs              # Main binary demonstrating examples
├── Cargo.toml               # Project dependencies and metadata
├── Makefile                 # Build automation and targets
└── README.md                # This file
```

## Examples Included

### 1. Async State Machine Example
Demonstrates how Rust transforms async functions into state machines with suspension points.

### 2. Multiple Awaits Example
Shows how a single async function can have multiple await points, creating multiple states in the generated state machine.

### 3. Variable Scoping Example
Illustrates how variables are handled across await boundaries and what gets stored in the Future's state.

### 4. Complex Async Function
Demonstrates async functions with parameters, error handling, and Result types.

### 5. HTTP Request Example
Real-world example using reqwest to make async HTTP requests.

### 6. Concurrent Execution
Shows how to run multiple async tasks concurrently using tokio::join!

## Testing

Run the test suite with:

```bash
make test
# or
cargo test
```

All examples include comprehensive unit tests to verify functionality.

## Continuous Integration

The project includes a GitHub Actions workflow (`.github/workflows/ci.yml`) that automatically:

1. Checks code formatting with rustfmt
2. Runs clippy linter with strict warnings
3. Executes all tests
4. Builds the release binary

The CI pipeline runs on every push and pull request to the main/master branch.

## Learning Resources

This project is designed to help you understand:

- How async/await works under the hood in Rust
- State machine transformations performed by the compiler
- Proper variable scoping in async contexts
- Best practices for async Rust development
- Integration with the tokio runtime and ecosystem

## Code Quality

All code in this repository:
- ✅ Follows Rust formatting guidelines (enforced by rustfmt)
- ✅ Passes clippy lints with no warnings
- ✅ Includes comprehensive comments explaining concepts
- ✅ Has associated unit tests
- ✅ Builds successfully in release mode

## Contributing

Contributions are welcome! Please ensure that:

1. Code is formatted: `make format`
2. All checks pass: `make ci`
3. New examples include tests
4. Code is well-commented

## License

This project is for educational purposes.

## Author

Danny Willems

