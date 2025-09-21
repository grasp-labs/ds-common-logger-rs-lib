# Contributing to DS Common Logger Rust Library

Thank you for your interest in contributing to the DS Common Logger Rust Library! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/your-username/ds-common-logger-rs-lib.git
   cd ds-common-logger-rs-lib
   ```

3. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/grasp-labs/ds-common-logger-rs-lib.git
   ```

## Development Setup

### Prerequisites

- Rust 1.76.0 or later
- No external dependencies required for basic development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Development Testing

```bash
# Run tests with different log levels
RUST_LOG=debug cargo test

# Run tests with JSON output
LOG_FORMAT=json cargo test

# Run specific test modules
cargo test logger::tests
```

## Making Changes

### Code Style

- Follow Rust conventions and use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Ensure all tests pass with `cargo test`
- Add tests for new functionality

### Commit Messages

Use clear, descriptive commit messages:

```text
feat: add support for custom log formatters
fix: resolve panic logging formatting issue
docs: update README with new logging examples
test: add integration tests for tracing initialization
perf: optimize JSON serialization performance
```

### Pull Request Process

1. **Create a feature branch** from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and ensure they pass all tests

3. **Update documentation** if you've added new features or changed APIs

4. **Push your branch** and create a pull request:

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Fill out the PR template** with a clear description of your changes

## Areas for Contribution

- **Bug fixes** - Report and fix issues with logging functionality
- **New features** - Add new logging capabilities (custom formatters, filters, etc.)
- **Documentation** - Improve examples, docs, and comments
- **Performance** - Optimize logging performance and memory usage
- **Testing** - Add more comprehensive tests for logging scenarios
- **Integration** - Add support for additional logging backends or formats
- **Configuration** - Enhance environment variable handling and validation
- **Error handling** - Improve error reporting and recovery mechanisms

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Library version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error messages
- Environment variables used (`RUST_LOG`, `LOG_FORMAT`)
- Log output examples (if applicable)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Questions?

Feel free to open an issue for questions or start a discussion. We're happy to help!
