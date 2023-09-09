use std::path::PathBuf;

/// Shock your ZFS pools to maintain good hygeine
#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Whether to recursively consider snapshots of each dataset
    #[arg(short, long)]
    pub recursive: bool,

    /// Path to the TOML configuration
    #[arg(short, long)]
    pub config: PathBuf,

    /// The pools or datasets to shock
    pub datasets: Vec<PathBuf>,
}
