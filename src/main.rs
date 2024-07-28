use anyhow::Result;
use clap::Parser;
use ripcards::{
    cli::{Cli, Subcommands::*},
    handlers::{handle_init, handle_new_card},
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.cmd {
        Some(Init) => handle_init()?,
        Some(New(args)) => handle_new_card(args)?,
        None => {}
    };

    Ok(())
}
