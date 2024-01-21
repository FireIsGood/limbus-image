//! CLI Argument parsing logic
use crate::config::{Config, SinnerData};
use clap::Parser;
use color_eyre::eyre::Context;
use color_eyre::Section;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    pub config: Option<String>,
}

/// Get the config given a file path.
///
/// Bubbles errors through `color_eyre`.
pub fn get_config() -> color_eyre::Result<Config> {
    // Get cli args
    let cli = Cli::parse();
    let config_path = PathBuf::from(cli.config.unwrap_or_else(|| "./config.toml".into()));

    // Get the relative root of the sinner folder file system
    let relative_root: String = config_path
        .as_path()
        .parent()
        .expect("Parent folder does not exist")
        .to_str()
        .expect("Folder name is somehow not usable as a string")
        .into();

    // Read the given config file
    let file_content = fs::read_to_string(config_path)
        .wrap_err("Failed to read config file")
        .suggestion("Add the path to your config file as an argument")?;
    let data: SinnerData = toml::from_str(&file_content)
        .wrap_err("Sinner data could not be parsed")
        .suggestion("Follow the config file format in `README.md`")?;
    let config = Config {
        data,
        relative_root,
    };

    Ok(config)
}
