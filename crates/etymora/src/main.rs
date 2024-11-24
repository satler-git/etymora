use clap::Parser;
use shadow_rs::shadow;

shadow!(build);

#[derive(Parser, Debug)]
#[command(version = build::CLAP_LONG_VERSION, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
}
