# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of DS Common Logger Rust Library
- High-performance structured logging with tracing integration
- Environment-driven log level filtering via `RUST_LOG`
- Dual output formats: compact and JSON via `LOG_FORMAT`
- Automatic panic logging as structured events
- Thread-safe initialization with idempotent behavior
- Comprehensive error context capture with span traces
- Zero-cost abstractions when logging is disabled

### Features

- **Environment-driven filtering** - Respects `RUST_LOG` environment variable
- **Dual output formats** - JSON and compact formats via `LOG_FORMAT` environment variable
- **Error context capture** - Automatic span trace capture for better debugging
- **Panic logging** - Converts panics to structured log events
- **Thread-safe initialization** - Idempotent initialization safe for concurrent use
- **Zero-cost abstractions** - Minimal runtime overhead when logging is disabled

### Dependencies

- `tracing` = "0.1" - Core tracing functionality
- `tracing-error` = "0.2" - Error context capture
- `tracing-subscriber` = "0.3" - Logging subscriber with env-filter, json, and fmt features

## [0.1.0] - 2025-09-21

### [0.1.0] Added

- Initial release
- Basic logging functionality with `init_tracing()` function
- Support for all standard log levels (trace, debug, info, warn, error)
- Structured logging with spans and events
- Panic hook integration for automatic panic logging
- Thread-safe global initialization
- Environment variable configuration
- Comprehensive documentation and examples

### [0.1.0] Security

- Memory-safe implementation using Rust's ownership system
- No unsafe code blocks
- Thread-safe operations without data races
- Secure environment variable handling

### [0.1.0] Documentation

- Complete API documentation with examples
- README with quick start guide
- Contributing guidelines
- Security policy
- Code of conduct
- Comprehensive test suite

---

## Security Information

Security-related changes will be documented in this changelog. For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
