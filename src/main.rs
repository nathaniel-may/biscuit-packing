mod biscuit_annealing;
mod cli;
mod point;
mod render;

use biscuit_annealing::approximate;
use clap::Parser;
use futures::future;
use tokio::task::JoinHandle;

#[derive(Clone, Debug)]
struct Run {
    biscuits: usize,
    width: f64,
    length: f64,
    iters: u64,
    announce_end: Option<String>,
}

struct App {
    header: String,
    runs: Vec<Run>,
}

impl App {
    fn jobs(&self) -> Vec<JoinHandle<()>> {
        self.runs
            .iter()
            .map(|r| {
                let biscuits = r.biscuits;
                let width = r.width;
                let length = r.length;
                let iters = r.iters;
                let announce_end = r.announce_end.clone();

                tokio::spawn(async move {
                    let points = approximate(biscuits, width, length, iters);
                    let rendered = render::render_packing(width, length, points);
                    let filename = format!("{biscuits}_biscuits_{width}X{length}_pan.svg");
                    svg::save(filename, &rendered).unwrap();
                    if let Some(msg) = announce_end {
                        println!("{}", msg)
                    }
                })
            })
            .collect()
    }
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let app = parse(args);
    println!("{}", app.header);
    future::join_all(app.jobs()).await;
    println!("done")
}

fn parse(args: cli::Args) -> App {
    // scale up so the svg comes out with reasonable dimensions
    // TODO do I need to do this scaling anymore????
    let width = args.pan_width * 10.0;
    let length = args.pan_length * 10.0;
    let iters = args.runs;

    match args.command {
        cli::Commands::Single { biscuits } => {
            let header = format!(
                "optimizing placement of {biscuits} biscuits on a {} X {} pan with {iters} runs",
                args.pan_width, args.pan_length
            );

            App {
                header,
                runs: vec![Run {
                    biscuits,
                    width,
                    length,
                    announce_end: None,
                    iters,
                }],
            }
        }
        // TODO check that start < end
        // TODO check that start >= 1 (zero would be ugly to implement)
        cli::Commands::Multi { start, end } => {
            let header = format!(
                "optimizing placement of biscuits from {start} to {end} on a {} X {} pan with {iters} runs",
                args.pan_width, args.pan_length
            );

            let mut runs = vec![];
            for n in start..(end + 1) {
                let announce_end = Some(format!("finished placing {n} biscuits"));
                let run = Run {
                    biscuits: n,
                    width,
                    length,
                    iters,
                    announce_end,
                };
                runs.push(run);
            }

            App { header, runs }
        }
    }
}
