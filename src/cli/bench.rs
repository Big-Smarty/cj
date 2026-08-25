use std::{num::NonZero, time::Duration};

use clap::Args;
use jiff::{Timestamp, Unit};

use crate::{algorithms, backends::BackendKind};

/// CJ is a crackjack worker with a cli and no network communication.
#[derive(Args, Debug)]
pub struct BenchArgs {
    /// Backend to select
    #[arg(value_enum, long)]
    backend: BackendKind,

    /// Algorithm to use
    #[arg(value_enum, long)]
    algorithm: algorithms::Algorithms,

    /// Batch size
    #[arg(long)]
    batch_size: NonZero<usize>,

    /// Warmup time
    #[arg(long, default_value_t = Duration::from_secs(10).into())]
    warmup_time: humantime::Duration,

    /// Sample time
    #[arg(long)]
    sample_time: humantime::Duration,

    /// Sample count
    #[arg(long)]
    sample_count: NonZero<usize>,

    /// Output file
    #[arg(long, default_value_t = format!("reports/{}.json", Timestamp::now().round(Unit::Second).unwrap().to_string()))]
    output: String,
}
