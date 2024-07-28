use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// RipCards: Flash Cards for your terminal
#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Cli {
    // Choose the command to run
    #[command(subcommand)]
    pub cmd: Option<Subcommands>,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Initialize repository with RipCards files and dir structures
    Init,
    /// Creates all basic cards files with default values
    New(NewCardArgs),
}

#[derive(Debug, Args)]
pub struct NewCardArgs {
    /// Path of the dir that will be set as a card. If not specified, executes
    /// in the current dir (where the command is executed)
    pub path: Option<PathBuf>,
}
