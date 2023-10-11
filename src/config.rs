use std::{fs, path::PathBuf};
use std::path::Path;
use serde::Deserialize;
use clap::Parser;

#[derive(Clone, Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub schedule: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub tasks: Vec<TaskConfig>,
}

impl Config {
    pub fn build(filename: &Path) -> Result<Config, &'static str> {
        let toml_string = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(_) => {
                return Err("Unable to read config file.");
            }
        };

        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                return Err("Unable to parse config file.");
            }
        };

        if config.tasks.len() == 0 {
            return Err("No tasks defined in the config file.");
        }

        Ok(config)
    }
}

/// Scheduler that sends Google Pub/Sub messages
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TOML configuration file
    #[arg(short, long, value_name = "FILE.toml")]
    pub config: PathBuf,
}

