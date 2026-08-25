//! CJ is a crackjack worker with a cli and no network communication.

use clap::Parser;

mod algorithms;
mod backend;
mod backends;
mod cli;
mod cracker;
mod device;
mod engine;
mod gpu_id_set;
mod pci_id;

fn main() {
    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::All);

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Bench(_bench_args) => println!("used bench command"),
        cli::Commands::Devices => println!("used devices command"),
        cli::Commands::Backends => println!("used backends command"),
    }
}
