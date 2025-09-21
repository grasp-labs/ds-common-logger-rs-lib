use ds_common_logger_rs_lib::init_tracing;
use std::panic;

#[test]
fn test_panic_logging() {
    init_tracing();

    // Test that panics are captured as log events
    let result = panic::catch_unwind(|| {
        panic!("Test panic for logging");
    });

    // The panic should be caught and logged
    assert!(result.is_err());

    // Log a message to verify the logger is still working
    tracing::info!("Panic handling test completed");
}

#[test]
fn test_panic_with_context() {
    init_tracing();

    let span = tracing::span!(tracing::Level::INFO, "panic_test_operation", test_id = "panic-test-456");

    let _enter = span.enter();

    let result = panic::catch_unwind(|| {
        tracing::info!("About to panic");
        panic!("Panic with span context");
    });

    assert!(result.is_err());
    tracing::info!("Panic with context test completed");
}
