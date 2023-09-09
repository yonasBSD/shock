use crate::config::{Config, PrefixConfig, TomlConfig};
use clap::Parser;
use core::fmt;
use std::{fs, process::exit};

mod cli;
mod config;

const NAME: &str = env!("CARGO_BIN_NAME");

fn main() {
    let args = cli::Args::parse();

    if args.datasets.is_empty() {
        bail("no datasets specified");
    }

    let config = {
        let toml = match fs::read_to_string(&args.config) {
            Ok(toml) => toml,
            Err(e) => bail(format_args!(
                "cannot read config file: {}: {e}",
                args.config.display()
            )),
        };

        let config = match toml::from_str::<TomlConfig>(&toml) {
            Ok(config) => config,
            Err(e) => bail(format_args!(
                "cannot parse config file: {}: {e}",
                args.config.display()
            )),
        };

        match Config::new(
            args.datasets,
            config
                .prefix
                .into_iter()
                .map(|(prefix, keep)| PrefixConfig { prefix, keep })
                .collect(),
        ) {
            Ok(config) => config,
            Err(overlapping_prefixes) => {
                overlapping_prefixes
                    .into_iter()
                    .for_each(|(a, b)| err(format_args!("overlapping prefix: \"{a}\", \"{b}\"")));
                exit(1);
            }
        }
    };
 }

fn err(msg: impl std::fmt::Display) {
    eprintln!("{NAME}: error: {msg}");
}

fn bail(msg: impl std::fmt::Display) -> ! {
    err(msg);
    exit(1);
}
