mod app;
mod biscuit_annealing;
mod cli;
mod point;
mod render;
#[cfg(test)]
mod test;

use clap::Parser;
use futures::future;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let app = app::parse(args);
    println!("{}", app.header);
    future::join_all(app.jobs()).await;
    println!("done")
}
