use clap::Parser;
use shadow_rs::shadow;
use tracing::{info, Level};

shadow!(build);

#[derive(Parser, Debug)]
#[command(version = build::CLAP_LONG_VERSION, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();
}
