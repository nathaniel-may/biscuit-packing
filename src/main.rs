mod biscuit_annealing;
mod cli;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
