use crate::spritesheet::SpriteSheetBuilder;
use image::{DynamicImage, Rgba};
use rusttype::point;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn run(
    ttf: PathBuf,
    text_files_dir: PathBuf,
    file_extension: String,
    fontmap_directory: PathBuf,
    font_scale: f32,
) -> Result<(), String> {
    // Validate the TTF file
    if !ttf.exists() {
        return Err(format!("TTF file does not exist: {:?}", ttf));
    }

    // Validate the TTF file is a TTF file
    if ttf.extension().unwrap().to_ascii_lowercase() != "ttf" {
        return Err(format!("TTF file is not a TTF file: {:?}", ttf));
    }

    // Validate the fontmap file ends with .fontmap_file
    if fontmap_directory.extension().is_some() {
        return Err("FONTMAP_DIRECTORY should be a directory".to_string());
    }

    // Load font
    let font = match rusttype::Font::try_from_vec(std::fs::read(&ttf).unwrap()) {
        Some(font) => font,
        None => return Err(format!("Failed to load font: {:?}", ttf)),
    };

    // Create directory
    let parent = fontmap_directory.parent().unwrap();
    std::fs::create_dir_all(&parent).unwrap();

    // Font details
    let scale = rusttype::Scale::uniform(font_scale);
    let colour = (255, 255, 255);
    let v_metrics = font.v_metrics(scale);

    // Build the characters to render
    let mut characters = HashSet::new();
    for entry in std::fs::read_dir(text_files_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("Path: {:?}", path);

        if path.is_file()
            && path
                .extension()
                .unwrap()
                .to_ascii_lowercase()
                .to_string_lossy()
                == file_extension
        {
            let text = std::fs::read_to_string(path).unwrap();
            for c in text.chars() {
                characters.insert(c);
            }
        }
    }

    // Render each character
    let mut images: HashMap<char, DynamicImage> = HashMap::new();
    let mut max_w = 0;
    let mut max_h = 0;

    for character in characters {
        let text = character.to_string();

        // layout the glyphs in a line with 20 pixels padding
        let padding = 8.0;
        let glyphs: Vec<_> = font
            .layout(&text, scale, point(padding, padding + v_metrics.ascent))
            .collect();

        // work out the layout size
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            // If the glyph has no pixel bounding box, then it's whitespace
            if glyphs
                .first()
                .map(|g| g.pixel_bounding_box().is_none())
                .unwrap_or(true)
            {
                (padding * 2.0) as u32
            } else {
                let min_x = glyphs
                    .first()
                    .map(|g| g.pixel_bounding_box().unwrap().min.x)
                    .unwrap();
                let max_x = glyphs
                    .last()
                    .map(|g| g.pixel_bounding_box().unwrap().max.x)
                    .unwrap();
                (max_x - min_x) as u32
            }
        };

        // Create a new rgba image with some padding
        let full_padding = (padding * 2.0) as u32;
        let mut image =
            DynamicImage::new_rgba8(glyphs_width + full_padding, glyphs_height + full_padding)
                .to_rgba8();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                    )
                });
            }
        }

        // Update max w and h
        let w = image.width();
        let h = image.height();
        if w > max_w {
            max_w = w;
        }
        if h > max_h {
            max_h = h;
        }

        // Insert image into hashmap
        let dynamic_image = DynamicImage::ImageRgba8(image);
        images.insert(character, dynamic_image);
    }

    // Assemble spritesheet
    let mut spritesheet = SpriteSheetBuilder::new("font_atlas".to_string());

    // Sort images for easier reading
    let mut images: Vec<_> = images.into_iter().collect();
    images.sort_by(|a, b| a.0.cmp(&b.0));

    // Add each image to the spritesheet
    for (character, image) in images {
        spritesheet.add_sprite(character.to_string(), image);
    }

    // Save
    spritesheet.save(&fontmap_directory)?;

    Ok(())
}
