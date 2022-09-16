#![forbid(unsafe_code)]

use clap::Parser;

mod error;
mod protocol;
mod proxy;

#[derive(Debug, Parser)]
#[clap(
    version = "v0.1.1",
    author = "Developed by @p4gefau1t (Page Fault)",
    about = "An unidentifiable mechanism that helps you bypass GFW"
)]
struct Args {
    #[clap(
        short,
        long,
        about,
        takes_value = true,
        help = ".toml config file name"
    )]
    config: String,
}

#[tokio::main]
async fn main() {
    let filename = Args::parse().config;
    if let Err(e) = proxy::launch_from_config_filename(filename).await {
        println!("failed to launch proxy: {}", e);
    }
}
