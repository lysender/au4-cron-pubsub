use std::process;
use clap::Parser;

use crate::config::{Args, Config};
use crate::run::run;

mod config;
mod error;
mod jwt;
mod pubsub;
mod run;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = Config::build(args.config.as_path()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run(config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
