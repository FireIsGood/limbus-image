//! Image generation and overlaying

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
) -> anyhow::Result<()> {
    // Open sinner image and set to the image size
    let mut sinner_portrait = image::open(input_image_path)?;
    sinner_portrait = resize_image(&sinner_portrait);

    // Add the text shadow overlay
    let wrapped_width: i32 = 15;
    let line_count = i32::try_from(textwrap::wrap(identity, wrapped_width as usize).len())
        .expect("Line count within i32 range");
    let overlay_file = format!("{}{}", overlay_path, line_count_to_overlay(line_count));
    let overlay = image::open(overlay_file)?;
    let overlay = resize_image(&overlay);
    image::imageops::overlay(&mut sinner_portrait, &overlay, 0, 0);

    // Add the rarity overlay
    let rarity_overlay_file = format!("{}{}", overlay_path, rarity_to_overlay(rarity));
    let rarity_overlay = image::open(rarity_overlay_file)?;
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
        wrapped_width,
        SHADOW_COLOR,
        &mut sinner_portrait,
        left_offset_top + shadow_offset,
        top_offset + shadow_offset,
    );
    write_text(
        name,
        wrapped_width,
        SHADOW_COLOR,
        &mut sinner_portrait,
        left_offset_bottom + shadow_offset,
        bottom_offset + shadow_offset,
    );

    // Actual text
    write_text(
        identity,
        wrapped_width,
        TEXT_COLOR,
        &mut sinner_portrait,
        left_offset_top,
        top_offset,
    );
    write_text(
        name,
        wrapped_width,
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
fn line_count_to_overlay(lines: i32) -> String {
    match lines {
        1 => "gradient_small.png",
        2 => "gradient_large.png",
        _ => panic!("Up to 2 lines of text should exist"),
    }
    .into()
}

/// Converts rarity to a string of the border asset to overlay
fn rarity_to_overlay(rarity: u8) -> String {
    match rarity {
        1 => "0.png",
        2 => "00.png",
        3 => "000.png",
        _ => panic!("Rarity must be 1 to 3"),
    }
    .into()
}

/// Writes wrapped text on to the image
fn write_text(
    text: &str,
    wrapped_width: i32,
    color: image::Rgba<u8>,
    img: &mut image::DynamicImage,
    x_offset: i32,
    y_offset: i32,
) {
    let x = x_offset;
    let mut y = y_offset;
    let font_size: f32 = 50.0;
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
