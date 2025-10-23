.PHONY: help build run format format-check lint clippy expand mir inspect clean test ci

# Default target
.DEFAULT_GOAL := help

## help: Display this help message
help:
	@echo "Rust Async/Await Course Example - Makefile Targets"
	@echo "===================================================="
	@echo ""
	@echo "Available targets:"
	@echo ""
	@grep -E '^## ' Makefile | sed 's/## /  /'
	@echo ""

## build: Build the project in release mode
build:
	@echo "Building project..."
	cargo build --release

## run: Run the project
run:
	@echo "Running project..."
	cargo run

## format: Format code using rustfmt
format:
	@echo "Formatting code..."
	cargo fmt

## format-check: Check if code is formatted correctly
format-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

## lint: Run clippy linter (alias for clippy target)
lint: clippy

## clippy: Run clippy linter with all warnings
clippy:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

## expand: Show macro expansion using cargo-expand (requires cargo-expand)
expand:
	@echo "Expanding macros..."
	@command -v cargo-expand >/dev/null 2>&1 || { echo "Installing cargo-expand..."; cargo install cargo-expand; }
	cargo expand

## mir: Display MIR (Mid-level Intermediate Representation) for the project
mir:
	@echo "Generating MIR..."
	@echo "Note: This requires nightly Rust. Switching to nightly for this command..."
	RUSTFLAGS="--emit=mir" cargo +nightly build
	@echo "MIR files generated in target/debug/deps/"
	@echo "To view MIR for a specific function, use: cargo +nightly rustc -- --emit=mir -Z dump-mir=all"

## inspect: Inspect the generated code (shows expanded async state machines)
inspect:
	@echo "Inspecting async transformations..."
	@command -v cargo-expand >/dev/null 2>&1 || { echo "Installing cargo-expand..."; cargo install cargo-expand; }
	@echo ""
	@echo "=== Expanded async_state_machine_example ==="
	cargo expand async_state_machine_example 2>/dev/null || cargo expand
	@echo ""
	@echo "To see assembly output, run: cargo rustc --release -- --emit asm"

## clean: Remove build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

## test: Run all tests
test:
	@echo "Running tests..."
	cargo test

## ci: Run all CI checks (format-check, lint, test, build)
ci: format-check clippy test build
	@echo ""
	@echo "✓ All CI checks passed!"
