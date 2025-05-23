#![warn(clippy::unwrap_used)]
#![warn(clippy::uninlined_format_args)]

pub mod cli;
pub mod provider;
pub mod reader;
pub mod utils;
pub mod writer;

use std::time::Instant;

use clap::Parser;
use humanize_duration::{prelude::DurationExt, Truncate};
use tracing::info;
use trin_utils::log::init_tracing_logger;

use crate::{reader::EpochReader, writer::EpochWriter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_logger();
    info!("Running E2HS writer");
    let config = cli::WriterConfig::parse();
    info!("With configuration: {config:?}");

    let start = Instant::now();
    let epoch_reader =
        EpochReader::new(config.epoch, config.epoch_acc_path, config.el_provider).await?;
    info!(
        "Time taken to download Era/Era1 and Receipts {}",
        start.elapsed().human(Truncate::Second)
    );

    let start = Instant::now();
    let epoch_writer = EpochWriter::new(config.target_dir, config.epoch);
    epoch_writer.write_epoch(epoch_reader)?;
    info!(
        "Time taken to finished writing blocks  {}",
        start.elapsed().human(Truncate::Second)
    );

    Ok(())
}
