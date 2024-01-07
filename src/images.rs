use image::DynamicImage;
use rusttype::{Font, Scale};

pub fn create_image(
    input_image_path: &str,
    output_image_path: &str,
    overlay_path: &str,
    rarity: u8,
    identity: &str,
    name: &str,
) -> anyhow::Result<()> {
    println!("Creating {} id: {}", name, identity);

    // Open sinner image and set to the image size
    let image_size: i32 = 600;
    let mut sinner_portrait = image::open(input_image_path)?;
    sinner_portrait = resize_image(sinner_portrait, 600);

    // Add the text shadow overlay
    let wrapped_width: i32 = 15;
    let line_count = textwrap::wrap(identity, wrapped_width as usize).len() as i32;
    let overlay_file = format!("{}{}", overlay_path, line_count_to_overlay(line_count));
    let overlay = image::open(overlay_file)?;
    let overlay = resize_image(overlay, 600);
    image::imageops::overlay(&mut sinner_portrait, &overlay, 0, 0);

    // Add the rarity overlay
    let rarity_overlay_file = format!("{}{}", overlay_path, rarity_to_overlay(rarity));
    let rarity_overlay = image::open(rarity_overlay_file)?;
    let rarity_overlay = resize_image(rarity_overlay, 600);
    image::imageops::overlay(&mut sinner_portrait, &rarity_overlay, 0, 0);

    // Add some text
    let top_offset = 14;
    let left_offset_top = 22;
    let left_offset_bottom = 71;
    let bottom_offset: i32 = image_size - 130;

    // Janky fake shadow
    let drop_shadow_color = image::Rgba::<u8>::from([30, 30, 30, 255]);
    let shadow_offset = 5;
    write_text(
        identity,
        wrapped_width,
        drop_shadow_color,
        &mut sinner_portrait,
        left_offset_top + shadow_offset,
        top_offset + shadow_offset,
    );
    write_text(
        name,
        wrapped_width,
        drop_shadow_color,
        &mut sinner_portrait,
        left_offset_bottom + shadow_offset,
        bottom_offset + shadow_offset,
    );

    // Actual text
    let color = image::Rgba::<u8>::from([255, 217, 0, 255]);
    write_text(
        identity,
        wrapped_width,
        color,
        &mut sinner_portrait,
        left_offset_top,
        top_offset,
    );
    write_text(
        name,
        wrapped_width,
        color,
        &mut sinner_portrait,
        left_offset_bottom,
        bottom_offset,
    );

    // Create parent folders
    let path = std::path::Path::new(output_image_path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    sinner_portrait.save(output_image_path).unwrap();
    // println!("Wrote file to {}", output_image_path);

    Ok(())
}
fn line_count_to_overlay(lines: i32) -> String {
    let path = match lines {
        1 => "gradient_small.png",
        2 => "gradient_large.png",
        _ => panic!("Up to 2 lines of text should exist"),
    }
    .into();
    path
}

fn rarity_to_overlay(rarity: u8) -> String {
    let path = match rarity {
        1 => "0.png",
        2 => "00.png",
        3 => "000.png",
        _ => panic!("Rarity must be 1 to 3"),
    }
    .into();
    path
}

fn write_text(
    text: &str,
    wrapped_width: i32,
    color: image::Rgba<u8>,
    img: &mut DynamicImage,
    x_offset: i32,
    y_offset: i32,
) {
    let x = x_offset;
    let mut y = y_offset;
    let font_size: f32 = 50.0;
    let line_height: i32 = (font_size * 1.5).round() as i32;

    // Load font data
    let font_bytes = include_bytes!("../font/ExcelsiorSans.ttf");
    let font = Font::try_from_bytes(font_bytes).unwrap();

    let wrapped_text = textwrap::wrap(text, wrapped_width as usize);

    for line in wrapped_text {
        imageproc::drawing::draw_text_mut(
            img,
            color,
            x,
            y,
            Scale {
                x: font_size,
                y: font_size,
            },
            &font,
            &line,
        );
        y = y + line_height;
    }
}

fn resize_image(image: DynamicImage, image_size: i32) -> DynamicImage {
    image.resize(
        image_size as u32,
        image_size as u32,
        image::imageops::FilterType::Gaussian,
    )
}
