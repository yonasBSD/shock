use crate::config::{Config, PrefixConfig, TomlConfig};
use anyhow::Context;
use clap::Parser;
use std::{
    fs, iter,
    ops::Not,
    process::{Command, Stdio},
};

mod cli;
mod config;
mod snapshot;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    if args.datasets.is_empty() {
        anyhow::bail!("no datasets specified");
    }

    let config = fs::read_to_string(&args.config)
        .with_context(|| format!("cannot read config file: {}", args.config.display()))
        .and_then(|toml| {
            toml::from_str::<TomlConfig>(&toml)
                .with_context(|| format!("cannot parse config file: {}", args.config.display()))
        })
        .and_then(|config| {
            Config::new(
                config
                    .prefix
                    .into_iter()
                    .map(|(prefix, keep)| PrefixConfig { prefix, keep })
                    .collect(),
            )
            .map_err(|overlapping_prefixes| {
                let mut iter = overlapping_prefixes
                    .into_iter()
                    .rev()
                    .map(|(a, b)| format!("overlapping prefix: \"{a}\", \"{b}\""));
                let first = iter.next().unwrap();
                iter.fold(anyhow::anyhow!(first), anyhow::Error::context)
                    .context("configuration has overlapping prefixes")
            })
        })?;

    let zfs_list_output = Command::new("zfs")
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
        .context("failed to run `zfs list`")
        .and_then(|c| {
            anyhow::ensure!(c.status.success(), "failed to run `zfs list`");
            Ok(c.stdout)
        })?;

    for snapshot in snapshot::to_delete(args.verbose, &config, &zfs_list_output) {
        let _ = Command::new("zfs")
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
            .context("failed to run `zfs destroy`")?
            .wait();
    }

    Ok(())
}
