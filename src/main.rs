//! Limbus company CLI tier list image generator
//!
//! It generates files based on a TOML config file you direct it to.

mod args;
mod config;
mod files;
mod images;

use crate::args::get_config;
use crate::files::iterate_sinners;

fn main() -> color_eyre::Result<()> {
    println!("Generating images...");
    color_eyre::install()?;
    // Timing function
    let time_before = std::time::Instant::now();

    let config_file = get_config()?;

    let sinners_generated = iterate_sinners(&config_file)?;

    let image_count_suffix = if sinners_generated == 1 { "" } else { "s" };
    let time_elapsed = time_before.elapsed().as_secs_f32();
    println!(
        "Generated {sinners_generated} image{image_count_suffix} in {time_elapsed:.2?} seconds!"
    );
    Ok(())
}
