//!
//! # DS Common Logger
//!
//! A high-performance, production-ready logging library for Rust applications that provides
//! structured logging with comprehensive tracing capabilities.
//!
//! ## Overview
//!
//! This library provides a simple yet powerful interface for initializing global tracing
//! subscribers with support for both JSON and compact output formats, environment-driven
//! filtering, and comprehensive error context capture.
//!
//! ## Features
//!
//! - **Environment-driven filtering** - Respects `RUST_LOG` environment variable
//! - **Dual output formats** - JSON and compact formats via `LOG_FORMAT` environment variable
//! - **Error context capture** - Automatic span trace capture for better debugging
//! - **Panic logging** - Converts panics to structured log events
//! - **Thread-safe initialization** - Idempotent initialization safe for concurrent use
//! - **Zero-cost abstractions** - Minimal runtime overhead when logging is disabled
//!
//! ## Quick Start
//!
//! ```rust
//! use ds_common_logger_rs_lib::init_tracing;
//!
//! // Initialize the global tracing subscriber
//! init_tracing();
//!
//! // Start logging
//! tracing::info!("Application started");
//! tracing::warn!("This is a warning");
//! tracing::error!("This is an error");
//! ```
//!
//! ## Configuration
//!
//! ### Log Level Filtering
//!
//! Control log levels using the `RUST_LOG` environment variable:
//!
//! ```bash
//! # Show all logs at info level and above
//! RUST_LOG=info cargo run
//!
//! # Show only error logs
//! RUST_LOG=error cargo run
//!
//! # Show debug logs for specific modules
//! RUST_LOG=debug,my_crate::module=info cargo run
//! ```
//!
//! ### Output Format
//!
//! Switch between compact and JSON output using the `LOG_FORMAT` environment variable:
//!
//! ```bash
//! # Compact format (default)
//! cargo run
//!
//! # JSON format
//! LOG_FORMAT=json cargo run
//! ```
//!
//! ## Thread Safety
//!
//! This library is designed to be thread-safe and can be safely called from multiple threads.
//! The initialization is idempotent, meaning it's safe to call `init_tracing()` multiple times.
//!
//! ## Performance
//!
//! The library is designed for minimal performance impact:
//! - Zero-cost when logging is disabled via environment filters
//! - Efficient string formatting and serialization
//! - Minimal memory allocations during normal operation
//!
use std::sync::Once;
use tracing::{error, info};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{self, format::FmtSpan},
    prelude::*,
    registry::Registry,
    util::SubscriberInitExt,
};

static INIT: Once = Once::new();

/// # Logger Module
///
/// This module provides functionality for initializing and configuring the global tracing subscriber.
///
/// The logger is designed to be initialized once per application and provides comprehensive
/// logging capabilities including structured logging, error context capture, and panic logging.
/// Initializes a global [`tracing`] subscriber exactly **once**.
///
/// This function sets up a comprehensive tracing subscriber with the following features:
///
/// - **Environment-driven filtering** - Uses `RUST_LOG` environment variable (defaults to `info`)
/// - **Flexible output format** - JSON or compact format based on `LOG_FORMAT` environment variable
/// - **Error context capture** - Adds [`ErrorLayer`] to capture span traces for better debugging
/// - **Panic logging** - Replaces the default panic hook to log panics as structured events
/// - **Idempotent initialization** - Safe to call multiple times from any thread
///
/// # Environment Variables
///
/// - `RUST_LOG`: Controls log level filtering (e.g., `info`, `debug`, `error`)
/// - `LOG_FORMAT`: Controls output format (`json` for JSON format, anything else for compact)
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from any thread. Subsequent calls after
/// the first successful initialization will be no-ops.
///
/// # Panics
///
/// This function will not panic, but if the tracing subscriber fails to initialize,
/// subsequent calls to tracing macros will be no-ops.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use ds_common_logger_rs_lib::init_tracing;
///
/// init_tracing();
/// tracing::info!("Application started");
/// ```
///
/// With environment variables:
///
/// ```bash
/// RUST_LOG=debug LOG_FORMAT=json cargo run
/// ```
///
/// In tests:
///
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_logging() {
///         init_tracing(); // Safe to call in tests
///         tracing::info!("Test log message");
///     }
/// }
/// ```
pub fn init_tracing() {
    INIT.call_once(|| {
        // 1. Filtering via env
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        // 2. Decide on format layer
        let use_json = std::env::var("LOG_FORMAT")
            .map(|v| v.eq_ignore_ascii_case("json"))
            .unwrap_or(false);

        // Base registry
        let base = Registry::default().with(env_filter).with(ErrorLayer::default());

        // 4. Attach stdout layer + init
        if use_json {
            base.with(
                fmt::layer()
                    .json()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_current_span(true)
                    .with_span_list(true)
                    .with_span_events(FmtSpan::CLOSE),
            )
            .init();
        } else {
            base.with(
                fmt::layer()
                    .compact()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(true)
                    .with_span_events(FmtSpan::CLOSE),
            )
            .init();
        }

        // 5. Log panics
        std::panic::set_hook(Box::new(|panic_info| {
            let location = panic_info
                .location()
                .map(|l| format!("{}:{}", l.file(), l.line()))
                .unwrap_or_else(|| "unknown".to_string());

            error!(
                %location,
                payload = %panic_info.to_string(),
                "Application panicked"
            );
        }));

        info!(
            format = %if use_json { "json" } else { "compact" },
            "Tracing initialized"
        );
    });
}

// endregion: <-- Logger
