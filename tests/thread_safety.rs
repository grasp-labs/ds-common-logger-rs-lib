use ds_common_logger_rs_lib::init_tracing;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_concurrent_initialization() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                // Each thread tries to initialize
                init_tracing();
                tracing::info!("Thread {} initialized", i);
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    tracing::info!("Concurrent initialization test completed");
}

#[test]
fn test_concurrent_logging() {
    init_tracing();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                for j in 0..10 {
                    tracing::info!("Thread {} message {}", i, j);
                    thread::sleep(Duration::from_millis(1));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    tracing::info!("Concurrent logging test completed");
}

#[test]
fn test_shared_logger_across_threads() {
    init_tracing();

    let shared_data = Arc::new("shared_data");

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&shared_data);
            thread::spawn(move || {
                tracing::info!("Thread {} processing {}", i, data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    tracing::info!("Shared logger test completed");
}
