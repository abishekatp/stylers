# Contributing to Stylers

Thank you for your interest in contributing to Stylers! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- **Rust Nightly**: This project requires nightly Rust due to the use of the `proc_macro_span` feature
  ```bash
  rustup toolchain install nightly
  ```
- The project uses the nightly toolchain specified in `rust-toolchain.toml`

### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/stylers.git
   cd stylers
   ```
3. The `rust-toolchain.toml` file will automatically use the correct nightly version

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

Examples are located in the `examples/` directory. To run an example:

```bash
cd examples/style
cargo run
```

## Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Run `cargo clippy` to check for common issues
- Ensure all tests pass before submitting a PR

## Project Structure

- `stylers_core/`: Core CSS parsing and scoping logic
- `stylers_macro/`: Procedural macros (`style!`, `style_sheet!`, etc.)
- `stylers/`: Main library crate with build script functionality
- `examples/`: Example projects demonstrating usage

## Making Changes

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and ensure:
   - Code compiles without warnings
   - All tests pass
   - Code is properly formatted (`cargo fmt`)
   - No clippy warnings (`cargo clippy`)

3. Update documentation if needed:
   - README.md for user-facing changes
   - Code comments for internal changes
   - CHANGELOG.md for notable changes

4. Commit your changes with clear, descriptive commit messages

5. Push to your fork and create a Pull Request

## Pull Request Process

1. Ensure your PR description clearly explains:
   - What changes were made
   - Why the changes were necessary
   - How to test the changes

2. Link any related issues

3. Ensure CI checks pass (if applicable)

4. Request review from maintainers

## Important Notes

- **Nightly Rust Requirement**: This project uses unstable features (`proc_macro_span`), so it requires nightly Rust. This is documented in the README.

- **Workspace Dependencies**: Internal dependencies (`stylers_core`, `stylers_macro`) should use path dependencies in `Cargo.toml` files, not version numbers, to ensure local development uses workspace members.

## Questions?

If you have questions or need help, please:
- Open an issue for bugs or feature requests
- Check existing issues and discussions
- Review the README for usage examples

Thank you for contributing to Stylers!

