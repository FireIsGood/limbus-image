use crate::config::{Config, SinnerData};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    pub config: Option<String>,
}

/// Get the config given a file path.
///
/// Bubbles errors through anyhow.
pub fn get_config() -> anyhow::Result<Config> {
    // Get cli args
    let cli = Cli::parse();
    let config_path = PathBuf::from(cli.config.unwrap_or_else(|| "./config.toml".into()));

    // Get the relative root of the sinner folder file system
    let relative_root: String = config_path
        .as_path()
        .parent()
        .expect("parent exists")
        .to_str()
        .expect("folder name usable as string")
        .into();

    // Read the given config file
    let file_content = fs::read_to_string(config_path)?;
    let data: SinnerData = toml::from_str(&file_content)?;
    let config = Config {
        data,
        relative_root,
    };

    Ok(config)
}
