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
    let config_path = PathBuf::from(cli.config.unwrap_or("./config.toml".into()));

    // Get the relative root of the sinner folder file system
    let relative_root = config_path.as_path().parent().unwrap().to_str().unwrap();
    let input_sinner_folder = format!("{}/input/sinners/", relative_root);
    let output_sinner_folder = format!("{}/output/sinners/", relative_root);
    let asset_folder = format!("{}/input/assets/", relative_root);

    // Read the given config file
    let file_content = fs::read_to_string(config_path)?;
    let data: SinnerData = toml::from_str(&file_content)?;
    let config = Config {
        data,
        input_sinner_folder,
        output_sinner_folder,
        asset_folder,
    };

    Ok(config)
}
