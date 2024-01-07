mod args;
mod config;
mod images;

use std::path::Path;

use crate::{args::get_config, images::create_image};

fn main() -> anyhow::Result<()> {
    println!("Hello, Sinners!\n");
    let config_file = get_config()?;

    // println!("Images are in: {}", config_file.input_sinner_folder);
    // println!("Misc. images are in: {}", config_file.asset_folder);
    // println!("Outptting to: {}", config_file.output_sinner_folder);

    for sinner in config_file.data.sinner.iter() {
        let input_image_folder = format!("{}{}", config_file.input_sinner_folder, sinner.path);

        // println!("Name: {}", sinner.name);

        for id in sinner.id.iter() {
            let input_id_image = format!("{}/id/{}", &input_image_folder, id.image);
            let output_id_image = format!(
                "{}{}_{}",
                &config_file.output_sinner_folder, sinner.path, id.image
            );

            // println!("Identity: {}", id.name);
            // println!("Path to input image: {}", &input_id_image);
            // println!("Path to output image: {}", &output_id_image);

            // Don't redo work
            let sinner_already_exists = Path::new(&output_id_image).exists();
            if sinner_already_exists {
                continue;
            };

            // Actual image stuff
            create_image(
                &input_id_image,
                &output_id_image,
                &config_file.asset_folder,
                id.rarity,
                &id.name,
                &sinner.name,
            )
            .map_err(|e| {
                println!("Oh no! The image {} could not be found.", input_id_image);
                e
            })?;
        }
    }

    Ok(())
}
