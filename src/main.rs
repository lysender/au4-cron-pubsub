use clap::Parser;
use std::process;

use crate::config::{Args, Config};
use crate::run::run;

mod config;
mod error;
mod jwt;
mod pubsub;
mod run;

// Re-exports
pub use error::{Error, Result};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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
