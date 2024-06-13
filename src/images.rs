//! Image generation and overlaying

use color_eyre::{eyre::Context, Section};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Identity image `{0}` was not found")]
    IdentityNotFound(String),
    #[error("Text shadow image `{0}` was not found")]
    TextShadowNotFound(String),
    #[error("Rarity image `{0}` was not found")]
    RarityNotFound(String),
    #[error("Names must be 1-2 lines")]
    TextTooLong,
    #[error("Rarity `{0}` is not allowed")]
    BadRarityLevel(i32),
}

/// Image size
///
/// This is fixed at 600 as the text size is set as constants
const IMAGE_SIZE: i32 = 600;

/// Text color
///
/// This is set across all images and is a yellow similar to in-game text
const TEXT_COLOR: image::Rgba<u8> = image::Rgba::<u8>([255, 217, 0, 255]);

/// Drop shadow color
///
/// This is just to add more contrast. The text writing library doesn't have the ability
/// to add shadows natively, so this is just another layer of text slightly offset
const SHADOW_COLOR: image::Rgba<u8> = image::Rgba::<u8>([30, 30, 30, 255]);

/// Wrapped text width
///
/// The number of characters before wrapping text in the image
const WRAPPED_WIDTH: u32 = 15;

/// Create an image of the sinner
///
/// Takes the image
pub fn create_image(
    input_image_path: &str,
    output_image_path: &str,
    overlay_path: &str,
    rarity: u8,
    identity: &str,
    name: &str,
) -> color_eyre::Result<()> {
    // Open sinner image and set to the image size
    let mut sinner_portrait = image::open(input_image_path)
        .wrap_err(ImageError::IdentityNotFound(input_image_path.into()))
        .suggestion(format!("Create the sinner image `{input_image_path}`"))?;
    sinner_portrait = resize_image(&sinner_portrait);

    // Calculate line count from character width
    let line_count = i32::try_from(textwrap::wrap(identity, WRAPPED_WIDTH as usize).len())
        .expect("Line count within i32 range");

    // We don't support anything above 2 lines
    if !(1..=2).contains(&line_count) {
        return Err(ImageError::BadRarityLevel(line_count))
            .suggestion("Change the configuration file to not have that rarity.");
    };

    // Add the text shadow overlay
    let overlay_file = format!("{}{}", overlay_path, line_count_to_overlay(line_count)?);
    let overlay = image::open(&overlay_file)
        .wrap_err(ImageError::TextShadowNotFound(overlay_file.clone()))
        .suggestion(format!("Create the shadow image `{overlay_file}`"))?;
    let overlay = resize_image(&overlay);
    image::imageops::overlay(&mut sinner_portrait, &overlay, 0, 0);

    // Add the rarity overlay
    let rarity_overlay_file = format!("{}{}", overlay_path, rarity_to_overlay(rarity));
    let rarity_overlay = image::open(&rarity_overlay_file)
        .wrap_err(ImageError::RarityNotFound(rarity_overlay_file.clone()))
        .suggestion(format!("Create the rarity image `{rarity_overlay_file}`"))?;
    let rarity_overlay = resize_image(&rarity_overlay);
    image::imageops::overlay(&mut sinner_portrait, &rarity_overlay, 0, 0);

    // Add some text
    let top_offset = 14;
    let left_offset_top = 22;
    let left_offset_bottom = 71;
    let bottom_offset: i32 = IMAGE_SIZE - 130;

    // Janky fake shadow
    let shadow_offset = 5;
    write_text(
        identity,
        WRAPPED_WIDTH,
        SHADOW_COLOR,
        &mut sinner_portrait,
        left_offset_top + shadow_offset,
        top_offset + shadow_offset,
    );
    write_text(
        name,
        WRAPPED_WIDTH,
        SHADOW_COLOR,
        &mut sinner_portrait,
        left_offset_bottom + shadow_offset,
        bottom_offset + shadow_offset,
    );

    // Actual text
    write_text(
        identity,
        WRAPPED_WIDTH,
        TEXT_COLOR,
        &mut sinner_portrait,
        left_offset_top,
        top_offset,
    );
    write_text(
        name,
        WRAPPED_WIDTH,
        TEXT_COLOR,
        &mut sinner_portrait,
        left_offset_bottom,
        bottom_offset,
    );

    // Create parent folders
    let path = std::path::Path::new(output_image_path);
    let prefix = path.parent().expect("parent exists");
    std::fs::create_dir_all(prefix).expect("ability to create directories");

    // Write the contents of this image to the Writer in PNG format.
    sinner_portrait.save(output_image_path).expect("file saved");

    Ok(())
}

/// Converts line count to a string of the gradient asset to overlay
fn line_count_to_overlay(lines: i32) -> color_eyre::Result<String> {
    match lines {
        1 => Ok("gradient_small.png".to_owned()),
        2 => Ok("gradient_large.png".to_owned()),
        _ => Err(ImageError::TextTooLong)
            .suggestion("Reduce the number of words or use acronyms in the name."),
    }
}

/// Converts rarity to a string of the border asset to overlay
fn rarity_to_overlay(rarity: u8) -> std::string::String {
    match rarity {
        1 => "0.png".to_owned(),
        2 => "00.png".to_owned(),
        3 => "000.png".to_owned(),
        _ => panic!("Somehow got a rarity out of range!"),
    }
}

/// Writes wrapped text on to the image
fn write_text(
    text: &str,
    wrapped_width: u32,
    color: image::Rgba<u8>,
    img: &mut image::DynamicImage,
    x_offset: i32,
    y_offset: i32,
) {
    let x = x_offset;
    let mut y = y_offset;
    let font_size: f32 = 50.0;
    // Line height is not going out of range, but if it does just use the saturated value
    #[allow(clippy::cast_possible_truncation)]
    let line_height: i32 = (font_size * 1.5).round() as i32;

    // Load font data
    let font_bytes = include_bytes!("../font/ExcelsiorSans.ttf");
    let font = rusttype::Font::try_from_bytes(font_bytes).expect("Font should exist");

    let wrapped_text = textwrap::wrap(text, wrapped_width as usize);

    for line in wrapped_text {
        imageproc::drawing::draw_text_mut(
            img,
            color,
            x,
            y,
            rusttype::Scale {
                x: font_size,
                y: font_size,
            },
            &font,
            &line,
        );
        y += line_height;
    }
}

/// Resize an image to the image size
///
/// Used to align everything correctly.
fn resize_image(image: &image::DynamicImage) -> image::DynamicImage {
    image.resize(
        IMAGE_SIZE as u32,
        IMAGE_SIZE as u32,
        image::imageops::FilterType::Gaussian,
    )
}
