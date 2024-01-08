use crate::config::{Config, Sinner};

pub fn iterate_sinners(config: &Config) -> anyhow::Result<i32> {
    let mut sinners_generated = 0;
    let sinner_list = &config.data.sinner;

    for sinner in sinner_list {
        let input_image_folder = sinner_folder(config, sinner);

        for id in &sinner.id {
            // Inputs are scoped as `input/sinners/[sinner]/[id]/*`
            let input_id_image = format!("{}/id/{}", &input_image_folder, id.image);
            // Outputs are scoped as `output/sinners/*`
            let output_id_image = format!("{}{}_{}", output_folder(config), sinner.path, id.image);

            let sinner_already_exists = std::path::Path::new(&output_id_image).exists();
            if sinner_already_exists {
                continue;
            };

            // Actual image stuff
            println!("Creating {} id: {}", &sinner.name, &id.name);

            // Create the image, returning any errors
            crate::images::create_image(
                &input_id_image,
                &output_id_image,
                &asset_folder(config),
                id.rarity,
                &id.name,
                &sinner.name,
            )
            .map_err(|e| {
                eprintln!("\nThe image at {input_id_image} was not found.");
                eprintln!("Check if the file exists!\n");
                e
            })?;

            sinners_generated += 1;
        }
    }

    Ok(sinners_generated)
}

/// Path to the sinner's folder from the input
fn sinner_folder(config: &Config, sinner: &Sinner) -> String {
    let root_sinners_folder = root_sinners_folder(config);
    format!("{}{}", root_sinners_folder, sinner.path)
}

/// Path to the root input sinner folder
fn root_sinners_folder(config: &Config) -> String {
    format!("{}/input/sinners/", config.relative_root)
}

/// Path to the root input assets folder
///
/// Assets must use the specific names as used in the Images function
fn asset_folder(config: &Config) -> String {
    format!("{}/input/assets/", config.relative_root)
}

/// Path to the output folder
///
/// Outputs are made in a flat directory as opposed to the inputs to make copying easier.
fn output_folder(config: &Config) -> String {
    format!("{}/output/sinners/", config.relative_root)
}
