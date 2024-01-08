//! Limbus company CLI tier list image generator
//!
//! It generates files based on a TOML config file you direct it to.

mod args;
mod config;
mod files;
mod images;

use crate::args::get_config;
use crate::files::iterate_sinners;

fn main() -> anyhow::Result<()> {
    println!("Generating images...");
    let config_file = get_config()?;

    let sinners_generated = iterate_sinners(&config_file)?;

    println!("Generated {sinners_generated} image(s)!");
    Ok(())
}
