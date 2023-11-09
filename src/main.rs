mod biscuit_annealing;
mod cli;
mod point;
mod render;

use biscuit_annealing::run;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let width = args.pan_width;
    let length = args.pan_length;
    let runs = args.runs;

    match args.command {
        cli::Commands::Single { biscuits } => {
            println!(
                "optimizing placement of {biscuits} biscuits on a {width} X {length} pan with {runs} runs"
            );

            let points = run(biscuits, width, length, runs);
            // for p in points {
            //     println!("{}, {}", p.x, p.y)
            // }
            let rendered = render::render_packing(args.pan_width, args.pan_length, points);
            let filename = format!("{biscuits}_biscuits_{width}X{length}_pan.svg");
            svg::save(filename, &rendered).unwrap();
        }
        // TODO check that start < end
        cli::Commands::Multi { start, end } => {
            println!(
                "optimizing placement of biscuits from {start} to {end} on a {width} X {length} pan with {runs} runs"
            );

            for n in start..(end + 1) {
                // println!(":: {n} biscuits ::");
                let points = run(n, width, length, runs);
                // for p in points {
                //     println!("{}, {}", p.x, p.y)
                // }
                let rendered = render::render_packing(args.pan_width, args.pan_length, points);
                let filename = format!("{n}_biscuits_{width}X{length}_pan.svg");
                svg::save(filename, &rendered).unwrap();
            }
        }
    }
}
