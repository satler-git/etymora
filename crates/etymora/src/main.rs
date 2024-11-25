mod dict_handler;
mod error;

use clap::Parser;
use shadow_rs::shadow;

use tracing::{info, Level};

use std::error::Error;
use std::time::Duration;

shadow!(build);

#[derive(Parser, Debug)]
#[command(version = build::CLAP_LONG_VERSION, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    Ok(())
}
