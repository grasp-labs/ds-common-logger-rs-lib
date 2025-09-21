# DS Common Logger Rust Library

[![Crates.io version](https://img.shields.io/crates/v/ds-common-logger-rs-lib.svg)](https://crates.io/crates/ds-common-logger-rs-lib)
[![Documentation](https://docs.rs/ds-common-logger-rs-lib/badge.svg)](https://docs.rs/ds-common-logger-rs-lib)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.76%2B-blue.svg)](https://www.rust-lang.org)

A high-performance, production-ready logging library for Rust applications that provides structured logging with comprehensive tracing capabilities.

## Features

- **Environment-driven filtering** - Respects `RUST_LOG` environment variable
- **Dual output formats** - JSON and compact formats via `LOG_FORMAT` environment variable
- **Error context capture** - Automatic span trace capture for better debugging
- **Panic logging** - Converts panics to structured log events
- **Thread-safe initialization** - Idempotent initialization safe for concurrent use
- **Zero-cost abstractions** - Minimal runtime overhead when logging is disabled

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ds-common-logger-rs-lib = "0.1.0"
```

Or use cargo add:

```sh
cargo add ds-common-logger-rs-lib
```

## Quick Start

### Basic Usage

```rust
use ds_common_logger_rs_lib::init_tracing;
use tracing::{info, warn, error, debug};

fn main() {
    // Initialize the global tracing subscriber
    init_tracing();

    // Start logging
    info!("Application started");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
}
```

### Structured Logging

```rust
use ds_common_logger_rs_lib::init_tracing;
use tracing::{info, span, Level};

fn main() {
    init_tracing();

    // Create a span for structured logging
    let span = span!(Level::INFO, "user_operation", user_id = 42, operation = "login");
    let _enter = span.enter();

    info!("User login attempt");
    info!("Login successful");

    // Span automatically exits when _enter is dropped
}
```

### Error Handling with Context

```rust
use ds_common_logger_rs_lib::init_tracing;
use tracing::{error, info, instrument};
use std::io;

#[instrument]
fn process_file(filename: &str) -> Result<String, io::Error> {
    info!("Processing file");

    match std::fs::read_to_string(filename) {
        Ok(content) => {
            info!("File read successfully");
            Ok(content)
        }
        Err(e) => {
            error!("Failed to read file: {}", e);
            Err(e)
        }
    }
}

fn main() {
    init_tracing();

    match process_file("config.toml") {
        Ok(_) => info!("File processing completed"),
        Err(e) => error!("File processing failed: {}", e),
    }
}
```

## Configuration

### Log Level Filtering

Control log levels using the `RUST_LOG` environment variable:

```bash
# Show all logs at info level and above
RUST_LOG=info cargo run

# Show only error logs
RUST_LOG=error cargo run

# Show debug logs for specific modules
RUST_LOG=debug,my_crate::module=info cargo run

# Show all logs for your crate, but only errors for dependencies
RUST_LOG=debug,my_crate=debug cargo run
```

### Output Format

Switch between compact and JSON output using the `LOG_FORMAT` environment variable:

```bash
# Compact format (default) - human-readable
cargo run

# JSON format - machine-readable
LOG_FORMAT=json cargo run
```

### Example Output

**Compact Format:**

```text
2024-01-15T10:30:45.123Z  INFO my_app: Application started
2024-01-15T10:30:45.124Z  WARN my_app: This is a warning message
2024-01-15T10:30:45.125Z ERROR my_app: This is an error message
```

**JSON Format:**

```json
{"timestamp":"2024-01-15T10:30:45.123Z","level":"INFO","target":"my_app","message":"Application started"}
{"timestamp":"2024-01-15T10:30:45.124Z","level":"WARN","target":"my_app","message":"This is a warning message"}
{"timestamp":"2024-01-15T10:30:45.125Z","level":"ERROR","target":"my_app","message":"This is an error message"}
```

## Advanced Usage

### Custom Spans and Events

```rust
use ds_common_logger_rs_lib::init_tracing;
use tracing::{info, span, event, Level};

fn main() {
    init_tracing();

    // Create a span with custom fields
    let span = span!(
        Level::INFO,
        "database_operation",
        table = "users",
        operation = "select",
        query_id = "q123"
    );

    let _enter = span.enter();

    // Log events within the span
    event!(Level::DEBUG, "Executing database query");
    event!(Level::INFO, "Query completed successfully");
}
```

### Panic Logging

The library automatically captures panics as structured log events:

```rust
use ds_common_logger_rs_lib::init_tracing;

fn main() {
    init_tracing();

    // This panic will be logged as a structured event
    panic!("Something went wrong!");
}
```

### Thread Safety

The library is thread-safe and can be used in multi-threaded applications:

```rust
use ds_common_logger_rs_lib::init_tracing;
use tracing::info;
use std::thread;

fn main() {
    init_tracing();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                info!("Thread {} started", i);
                // Do some work
                info!("Thread {} finished", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

## Environment Variables

The library uses the following environment variables:

- `RUST_LOG` - Controls log level filtering (e.g., `info`, `debug`, `error`)
- `LOG_FORMAT` - Controls output format (`json` for JSON format, anything else for compact)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
