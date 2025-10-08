///
/// cargo run --example sync_logger
/// LOG_FORMAT=json cargo run --example sync_logger
/// RUST_LOG=debug cargo run --example sync_logger
///
use ds_common_logger_rs_lib::init_tracing;
use tracing::{error, info, Level};

fn main() {
    init_tracing();

    info!("Application started");
    match helper("test") {
        Ok(_) => info!("Helper function called"),
        Err(e) => error!("Unable to call: {}", e),
    }
    info!("Application finished");
}

#[tracing::instrument(
    level = Level::INFO,
    fields(
        arg = %arg
    )
)]
fn helper(arg: &str) -> Result<String, std::io::Error> {
    info!("Helper function called with arg: {}", arg);
    Err(std::io::Error::other(arg))
}
