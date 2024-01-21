//! File accessing and general driver stuff
use color_eyre::eyre::Context;

use crate::config::{Config, Sinner};

/// Iterates each sinner and each identity for all the work
///
/// Images are only created if they do not already exist in the output directory.
/// Errors are bubbled up through a result. Returns the number generated.
pub fn iterate_sinners(config: &Config) -> color_eyre::Result<i32> {
    let mut sinners_generated = 0;
    let sinner_list = &config.data.sinner;

    for (index, sinner) in sinner_list.iter().enumerate() {
        let input_image_folder = sinner_folder(config, sinner);

        generate_ids(
            sinner,
            index,
            &input_image_folder,
            config,
            &mut sinners_generated,
        )
        .wrap_err(format!("Sinner `{}` could not be generated", sinner.name))?;
    }

    Ok(sinners_generated)
}

/// Generate images for every ID of a sinner
fn generate_ids(
    sinner: &Sinner,
    sinner_index: usize,
    input_image_folder: &str,
    config: &Config,
    sinners_generated: &mut i32,
) -> color_eyre::Result<()> {
    for (id_index, id) in sinner.id.iter().enumerate() {
        let input_id_image = input_sinner_id(input_image_folder, id);
        let output_id_image = output_sinner_id(config, sinner, sinner_index, id, id_index);

        let sinner_already_exists = std::path::Path::new(&output_id_image).exists();
        if sinner_already_exists {
            continue;
        };

        // Actual image stuff
        println!(
            "Creating ({:02}) {} id #{:02}: {}",
            sinner_index + 1,
            &sinner.name,
            id_index + 1,
            &id.name
        );

        // Create the image, returning any errors
        crate::images::create_image(
            &input_id_image,
            &output_id_image,
            &asset_folder(config),
            id.rarity,
            &id.name,
            &sinner.name,
        )
        .wrap_err(format!("Identity `{}` could not be generated", id.name))?;

        *sinners_generated += 1;
    }

    Ok(())
}

/// Path to the root input sinner folder
fn root_sinners_folder(config: &Config) -> String {
    format!("{}/input/", config.relative_root)
}

/// Path to the root input assets folder
///
/// Assets must use the specific names as used in the Images function
fn asset_folder(config: &Config) -> String {
    format!("{}/asset/", config.relative_root)
}

/// Path to the sinner's folder from the input
fn sinner_folder(config: &Config, sinner: &Sinner) -> String {
    let root_sinners_folder = root_sinners_folder(config);
    format!("{}{}", root_sinners_folder, sinner.path)
}

/// Path to the sinner's ID input image
fn input_sinner_id(input_image_folder: &str, id: &crate::config::Identity) -> String {
    // Inputs are scoped as `input/sinners/[sinner]/[id]/*`
    format!("{}/id/{}", &input_image_folder, id.image)
}

/// Path to the output folder
///
/// Outputs are made in a flat directory as opposed to the inputs to make copying easier.
fn output_folder(config: &Config) -> String {
    format!("{}/output/", config.relative_root)
}

/// Path to the sinner's ID output image
fn output_sinner_id(
    config: &Config,
    sinner: &Sinner,
    sinner_index: usize,
    id: &crate::config::Identity,
    id_index: usize,
) -> String {
    // Outputs are scoped as `output/id/*`
    format!(
        "{}id/{:02}_{}_{:02}_{}",
        output_folder(config),
        sinner_index + 1,
        sinner.path,
        id_index + 1,
        id.image
    )
}
