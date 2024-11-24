use clap::Parser;
use shadow_rs::shadow;
use tracing::{info, Level};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(feature = "jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
