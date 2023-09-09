use std::path::PathBuf;

/// Shock your ZFS pools to maintain good hygeine
#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Recursively operate on the specified datasets
    #[arg(short, long)]
    pub recursive: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Path to the TOML configuration
    #[arg(short, long)]
    pub config: PathBuf,

    /// The pools or datasets to shock
    pub datasets: Vec<PathBuf>,
}
