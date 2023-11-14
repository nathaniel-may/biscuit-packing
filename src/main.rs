use biscuit_packing::app;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match app::run().await {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}
