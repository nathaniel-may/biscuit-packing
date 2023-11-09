use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of biscuits to place on the pan
    #[arg(short = 'n', long)]
    pub biscuits: usize,

    /// Pan width
    #[arg(short = 'w', long)]
    pub pan_width: f64,

    /// Pan length
    #[arg(short = 'l', long)]
    pub pan_length: f64,

    /// Number of simulated annealing runs
    #[arg(short, long, default_value_t = 5000000)]
    pub runs: u64,
}
