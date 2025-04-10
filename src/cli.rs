// src/cli.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 't', long)]
    pub target: f64,
    #[arg(short = 'd', long, default_value_t = 6)]
    pub max_depth: usize,
    #[arg(short = 'e', long, default_value_t = 1.0)]
    pub tolerance: f64,
    #[arg(short = 'o', long)]
    pub output: Option<String>,
    #[arg(short = 'c', long, default_value_t = 2 ^ 16)]
    pub chunk_size: usize,
    #[arg(short = 'n', long)]
    pub num_threads: Option<usize>,
}
