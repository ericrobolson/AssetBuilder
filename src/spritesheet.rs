use image::{DynamicImage, GenericImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// A rectangle that contains an image and its position in the sprite sheet.
/// This is used to pack the images into the sprite sheet.
struct Rect {
    animation: String,
    frame_index: usize,
    image: DynamicImage,
    x: u32,
    y: u32,
}

/// A sprite sheet that contains a collection of sprites and an image
pub struct SpriteSheetBuilder {
    sheet: SpriteSheet,
    sprites_to_add: Vec<Rect>,
}
impl SpriteSheetBuilder {
    /// Create a new sprite sheet builder. The width and height will be resized to be a power of 2 automatically.
    pub fn new(name: String) -> Self {
        Self {
            sheet: SpriteSheet {
                width: 0,
                height: 0,
                name,
                sprites: HashMap::new(),
            },
            sprites_to_add: vec![],
        }
    }

    /// Add a sprite to the sprite sheet.
    pub fn add_sprite(&mut self, animation_name: String, img: DynamicImage) {
        // Crop image and calculate offsets for drawing a centered image
        let original_width = img.width();
        let original_height = img.height();

        let mut min_x = img.width() - 1;
        let mut min_y = img.height() - 1;
        let mut max_x = 0;
        let mut max_y = 0;

        for x in 0..img.width() {
            for y in 0..img.height() {
                let pixel = img.get_pixel(x, y);
                if pixel[3] != 0 {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        // crop image
        let mut top_left_offset_x = min_x;
        let mut top_left_offset_y = min_y;

        let img_is_empty = min_x == img.width() - 1 || min_y == img.height() - 1;
        let image = if img_is_empty {
            // Pretty weak but we'll just return an empty image.
            // This way we can still add the frame to the sheet
            // and the real edge case is that space will show up
            // the right size in font maps.
            let w = img.width() / 2;
            let h = img.height() / 2;

            top_left_offset_x = w;
            top_left_offset_y = h;
            DynamicImage::new_rgba8(w, h) // empty
        } else {
            let mut image = img;
            image = image.crop(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1);

            image
        };

        // Make sure sheet exists
        if self.sheet.sprites.get(&animation_name).is_none() {
            self.sheet.sprites.insert(animation_name.clone(), vec![]);
        }

        let animation = self.sheet.sprites.get_mut(&animation_name).unwrap();
        let idx = animation.len();

        let width = image.width();
        let height = image.height();

        // Now update that animation
        animation.push(Frame {
            top_left_offset_x,
            top_left_offset_y,
            x: 0,
            y: 0,
            width,
            height,
            original_width,
            original_height,
            center_offset_x: (original_width / 2) as i32 - (min_x as i32),
            center_offset_y: (original_height / 2) as i32 - (min_y as i32),
        });

        // Add sprite to list of sprites to add
        self.sprites_to_add.push(Rect {
            animation: animation_name.clone(),
            frame_index: idx,
            image: image,
            x: 0,
            y: 0,
        });
    }

    pub fn save(&mut self, path: &PathBuf) -> Result<(), String> {
        if path.extension().is_some() {
            return Err(format!(
                "Path {:?} should not have an extension when saving a sprite sheet",
                path
            ));
        }

        // We'll pack the atlas by using the naive algorithm of sorting by height.
        // I got better things to do.
        // https://www.david-colson.com/2020/03/10/exploring-rect-packing.html

        // Sort rectangles by img height
        self.sprites_to_add
            .sort_by(|a, b| b.image.height().cmp(&a.image.height()));

        let max_columns = (self.sprites_to_add.len() as f32).sqrt().ceil() as u32;

        // Get average width and height of all images
        let avg_width = self
            .sprites_to_add
            .iter()
            .map(|r| r.image.width())
            .sum::<u32>()
            / self.sprites_to_add.len() as u32;

        let avg_height = self
            .sprites_to_add
            .iter()
            .map(|r| r.image.height())
            .sum::<u32>()
            / self.sprites_to_add.len() as u32;

        // Scale width to be a power of 2 that could contain roughly a quarter of the sprites
        let target_width = avg_width * max_columns;
        let mut width = 2;
        while width < target_width {
            width *= 2;
        }

        let mut height = 2;
        while height < avg_height {
            height *= 2;
        }

        // Now go through and pack all the rectangles by setting x and y
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut largest_height_this_row = 0;
        for rect in self.sprites_to_add.iter_mut() {
            if x_pos + rect.image.width() > width {
                x_pos = 0;
                y_pos += largest_height_this_row;
                largest_height_this_row = 0;
            }

            if y_pos + rect.image.height() > height {
                height *= 2;
            }

            rect.x = x_pos;
            rect.y = y_pos;

            x_pos += rect.image.width();

            if rect.image.height() > largest_height_this_row {
                largest_height_this_row = rect.image.height();
            }
        }

        // Write rectangles to the image as well as frame data
        let mut image = DynamicImage::new_rgba8(width, height);
        self.sheet.width = width;
        self.sheet.height = height;

        for sprite in self.sprites_to_add.iter() {
            image.copy_from(&sprite.image, sprite.x, sprite.y).unwrap();

            // write rectangle coordinates to frame
            let frame =
                &mut self.sheet.sprites.get_mut(&sprite.animation).unwrap()[sprite.frame_index];

            frame.x = sprite.x;
            frame.y = sprite.y;
        }

        // Create directory if it doesn't exist
        if !path.is_dir() {
            std::fs::create_dir_all(&path).unwrap();
        }

        let path = path.join(&self.sheet.name);

        // Save image
        let img_path = format!("{}.png", path.as_os_str().to_str().unwrap());
        image.save(&img_path).unwrap();

        // Save json
        let json_path = format!("{}.json", path.as_os_str().to_str().unwrap());
        let json = serde_json::to_string_pretty(&self.sheet).unwrap();
        std::fs::write(&json_path, json).unwrap();

        println!("Saved JSON to {:?}", json_path);
        println!("Saved sprite sheet to {:?}", img_path);
        println!("Width: {}, Height: {}", width, height);

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpriteSheet {
    /// The width of the sprite sheet
    pub width: u32,
    /// The height of the sprite sheet
    pub height: u32,
    /// The name of the sprite sheet
    pub name: String,
    /// The sprites in the sprite sheet
    pub sprites: HashMap<String, Vec<Frame>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub top_left_offset_x: u32,
    pub top_left_offset_y: u32,
    pub center_offset_x: i32,
    pub center_offset_y: i32,
    /// The x position of the frame in the sprite sheet
    pub x: u32,
    /// The y position of the frame in the sprite sheet
    pub y: u32,
    /// The width of the frame
    pub width: u32,
    /// The height of the frame
    pub height: u32,
    /// The original width of the frame before cropping
    pub original_width: u32,
    /// The original height of the frame before cropping
    pub original_height: u32,
}
