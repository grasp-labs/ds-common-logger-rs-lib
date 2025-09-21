use ds_common_logger_rs_lib::init_tracing;
use tracing::{debug, error, info, warn};

#[test]
fn test_tracing_initialization() {
    // This should not panic
    init_tracing();

    // Test that we can log messages
    info!("Test info message");
    warn!("Test warning message");
    error!("Test error message");
    debug!("Test debug message");
}

#[test]
fn test_multiple_initialization_calls() {
    // Multiple calls should be safe (idempotent)
    init_tracing();
    init_tracing();
    init_tracing();

    info!("Multiple initialization test passed");
}

#[test]
fn test_structured_logging() {
    init_tracing();

    let span = tracing::span!(tracing::Level::INFO, "test_operation", operation_id = "test-123", user_id = 42);

    let _enter = span.enter();
    info!("Operation started");
    info!("Operation completed");
}

#[test]
fn test_log_levels() {
    init_tracing();

    // Test all log levels
    tracing::trace!("Trace message");
    tracing::debug!("Debug message");
    tracing::info!("Info message");
    tracing::warn!("Warning message");
    tracing::error!("Error message");
}
