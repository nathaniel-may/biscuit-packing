mod biscuit_annealing;
mod cli;

use biscuit_annealing::run;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    let biscuits = args.biscuits;
    let width = args.pan_width;
    let length = args.pan_length;
    let runs = args.runs;
    println!(
        "optimizing placement of {biscuits} biscuits on a {width} X {length} pan with {runs} runs"
    );

    let points = run(biscuits, width, length, runs);
    for p in points {
        println!("{}, {}", p.x, p.y)
    }
}
