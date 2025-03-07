use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::Result;

#[derive(Clone, Debug, Deserialize)]
pub struct TaskConfig {
    pub name: String,
    pub schedule: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PubSubConfig {
    pub key_file: String,
    pub topic: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub tasks: Vec<TaskConfig>,
    pub jwt_secret: String,
    pub pubsub: PubSubConfig,
}

impl Config {
    pub fn build(filename: &Path) -> Result<Config> {
        let toml_string = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(_) => {
                return Err("Unable to read config file.".into());
            }
        };

        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                return Err("Unable to parse config file.".into());
            }
        };

        if config.tasks.len() == 0 {
            return Err("No tasks defined in the config file.".into());
        }

        // Validate if key file exists
        let config_path = Path::new(config.pubsub.key_file.as_str());
        if !config_path.exists() {
            return Err("Service account file does not exists.".into());
        }

        // Validate if jwt_secret has enough string length
        if config.jwt_secret.len() < 1 {
            return Err("JWT secret must not be empty.".into());
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
