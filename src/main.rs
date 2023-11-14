mod app;
// TODO move all these under the a lib module
mod biscuit_annealing;
mod cli;
mod error;
mod point;
mod render;
#[cfg(test)]
mod test;

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
