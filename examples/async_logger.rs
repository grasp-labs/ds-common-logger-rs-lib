///
/// cargo run --example async_logger
/// LOG_FORMAT=json cargo run --example async_logger
/// RUST_LOG=debug cargo run --example async_logger
///
use ds_common_logger_rs_lib::init_tracing;
use tracing::{debug, error, info, warn, Level};

#[tokio::main]
async fn main() {
    init_tracing();

    info!("Application started");
    debug!("Debug message");
    match helper1("test").await {
        Ok(_) => info!("Helper function called"),
        Err(e) => error!("Unable to to call: {}", e),
    }

    match helper2("test").await {
        Ok(_) => info!("Helper function called"),
        Err(e) => error!("Unable to to call: {}", e),
    }

    info!("Application finished");
}

#[tracing::instrument(
    ret(level = Level::WARN)
    fields(arg = %arg)
)]
async fn helper1(arg: &str) -> Result<String, std::io::Error> {
    warn!("Helper function called");
    Err(std::io::Error::other(arg))
}

#[tracing::instrument(
    ret(level = Level::INFO)
    fields(arg = %arg)
)]
async fn helper2(arg: &str) -> Result<String, std::io::Error> {
    info!("Helper2 function called with arg: {}", arg);
    Ok("test".to_string())
}
