use clap::Parser;
use ripcards::cli::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:#?}", cli);
}
