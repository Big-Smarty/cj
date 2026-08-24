//! CJ is a crackjack worker with a cli and no network communication.

use clap::Parser;

mod algorithms;
mod backends;
mod cli;
mod engine;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Bench(_bench_args) => println!("used bench command"),
        cli::Commands::Devices => println!("used devices command"),
        cli::Commands::Backends => println!("used backends command"),
    }
}
