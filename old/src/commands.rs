use clap::{Parser, Subcommand};

/// spotbin - Spotlight importer for developers
#[derive(Parser)]
#[command()]
pub struct MainCommand {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Dump(Dump),
}

/// Dump binary info.
#[derive(Parser)]
pub struct Dump {
    /// Binary input file.
    pub in_file: std::path::PathBuf,
}
