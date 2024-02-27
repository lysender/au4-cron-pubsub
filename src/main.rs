use clap::Parser;
use std::process;

use crate::config::{Args, Config};
use crate::run::run;

mod config;
mod error;
mod jwt;
mod pubsub;
mod run;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "cron_pubsub=info")
    }

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
