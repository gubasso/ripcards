use anyhow::Result;
use clap::Parser;
use ripcards::{
    cli::{Cli, SessionSubcommands::*, Subcommands::*},
    handlers::{handle_init, handle_new_card, handle_session_progress, handle_session_start},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Some(Init) => handle_init()?,
        Some(New(args)) => handle_new_card(args)?,
        Some(Session(Start(args))) => handle_session_start(args)?,
        Some(Session(Progress(args))) => handle_session_progress(args)?,
        None => {}
    };

    Ok(())
}
