use clap::{Parser, Subcommand};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pan width
    #[arg(short = 'w', long)]
    pub pan_width: f64,

    /// Pan length
    #[arg(short = 'l', long)]
    pub pan_length: f64,

    /// Number of simulated annealing runs
    #[arg(short, long, default_value_t = 5000000)]
    pub runs: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Runs optimization for a set number of biscuits
    Single {
        /// Number of biscuits to pack
        #[arg(short = 'n', long)]
        biscuits: usize,
    },
    /// Runs optimizations for multiple counts of biscuits
    Multi {
        /// Number of biscuits to start packing (inclusive)
        #[arg(long)]
        start: usize,

        /// Number of biscuits to end packing (inclusive)
        #[arg(long)]
        end: usize,
    },
}
