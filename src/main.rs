use crate::config::{Config, PrefixConfig, TomlConfig};
use clap::Parser;
use core::fmt;
use std::{
    fs, iter,
    ops::Not,
    process::{exit, Command, Stdio},
};

mod cli;
mod config;
mod snapshot;

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

    let zfs_list_output = match Command::new("zfs")
        .args(
            iter::once("list")
                .chain(args.recursive.then_some("-r"))
                .chain(["-Hpt", "snap", "-o", "name", "-S", "creation"]),
        )
        .args(args.datasets)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .output()
    {
        Ok(c) if c.status.success() => c.stdout,
        Ok(_) => bail(format_args!("failed to run `zfs list`")),
        Err(e) => bail(format_args!("failed to run `zfs list`: {e}")),
    };

    for snapshot in snapshot::to_delete(args.verbose, &config, &zfs_list_output) {
        Command::new("zfs")
            .args(
                ["destroy", "-v"]
                    .into_iter()
                    .chain(args.destroy.not().then_some("-n")),
            )
            .arg(&snapshot)
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn err(msg: impl fmt::Display) {
    eprintln!("{NAME}: error: {msg}");
}

fn bail(msg: impl fmt::Display) -> ! {
    err(msg);
    exit(1);
}
