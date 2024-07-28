use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// Study Session management
    #[command(subcommand)]
    Session(SessionSubcommands),
}

#[derive(Debug, Args)]
pub struct NewCardArgs {
    /// Path of the dir that will be set as a card. If not specified, executes
    /// in the current dir (where the command is executed)
    pub path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum SessionSubcommands {
    Start(SessionMethodArgs),
    Progress(SessionProgressArgs),
}

#[derive(Debug, Args)]
pub struct SessionMethodArgs {
    /// Select the spaced repetition method for this study session
    #[arg(long, short)]
    pub method: Option<MethodOptions>,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum MethodOptions {
    Leitner,
}

#[derive(Debug, Args)]
pub struct SessionProgressArgs {
    /// Select the spaced repetition method for this study session
    #[arg(long, short)]
    compact: MethodOptions,
    #[command(flatten)]
    pub method_args: SessionMethodArgs,
}
