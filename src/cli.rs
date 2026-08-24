use clap::{Parser, Subcommand};

mod bench;

/// CJ is a crackjack worker with a cli and no network communication.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Bench(bench::BenchArgs),
    Devices,
}
