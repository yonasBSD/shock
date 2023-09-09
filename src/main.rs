use clap::Parser;
use std::process::exit;

mod cli;
mod config;

const NAME: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = cli::Args::parse();

    if args.datasets.is_empty() {
        let name = std::env::args().next().unwrap();
        eprintln!("{NAME}: error: no datasets specified");
        exit(1);
    }
}
