mod biscuit_annealing;
mod cli;
mod point;
mod render;

use biscuit_annealing::run;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    // scale up so the svg comes out with reasonable dimensions
    let width = args.pan_width * 10.0;
    let length = args.pan_length * 10.0;
    let runs = args.runs;

    match args.command {
        cli::Commands::Single { biscuits } => {
            println!(
                "optimizing placement of {biscuits} biscuits on a {} X {} pan with {runs} runs",
                args.pan_width, args.pan_length
            );

            let points = run(biscuits, width, length, runs);
            for p in &points {
                println!("{}, {}", p.x, p.y)
            }
            let rendered = render::render_packing(width, length, points);
            let filename = format!(
                "{biscuits}_biscuits_{}X{}_pan.svg",
                args.pan_width, args.pan_length
            );
            svg::save(filename, &rendered).unwrap();
        }
        // TODO check that start < end
        // TODO check that start >= 1 (zero would be ugly to implement)
        cli::Commands::Multi { start, end } => {
            println!(
                "optimizing placement of biscuits from {start} to {end} on a {} X {} pan with {runs} runs",
                args.pan_width, args.pan_length
            );

            for n in start..(end + 1) {
                println!("finished placing {n} biscuits");
                let points = run(n, width, length, runs);
                let rendered = render::render_packing(width, length, points);
                let filename = format!(
                    "{n}_biscuits_{}X{}_pan.svg",
                    args.pan_width, args.pan_length
                );
                svg::save(filename, &rendered).unwrap();
            }
            println!("done")
        }
    }
}
