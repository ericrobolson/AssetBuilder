use image::{DynamicImage, GenericImage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// A sprite sheet that contains a collection of sprites and an image
pub struct SpriteSheetBuilder {
    pub sheet: SpriteSheet,
    pub image: DynamicImage,
}
impl SpriteSheetBuilder {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            sheet: SpriteSheet {
                width,
                height,
                name,
                sprites: HashMap::new(),
            },
            image: DynamicImage::new_rgba8(width, height),
        }
    }

    /// Add a sprite to the sprite sheet.
    pub fn add_sprite(&mut self, name: String, x: u32, y: u32, img: DynamicImage) {
        let width = img.width();
        let height = img.height();
        if width > self.sheet.width {
            panic!(
                "Image width {} is greater than the sprite sheet width {}",
                width, self.sheet.width
            );
        }
        if height > self.sheet.height {
            panic!(
                "Image height {} is greater than the sprite sheet height {}",
                height, self.sheet.height
            );
        }

        self.write_to_image(x, y, &img);

        if self.sheet.sprites.get(&name).is_none() {
            self.sheet.sprites.insert(name.clone(), vec![]);
        }

        self.sheet.sprites.get_mut(&name).unwrap().push(Frame {
            x,
            y,
            width,
            height,
        });
    }

    pub fn save(&self, path: PathBuf) -> Result<(), String> {
        if path.extension().is_some() {
            return Err(format!(
                "Path {:?} should not have an extension when saving a sprite sheet",
                path
            ));
        }

        if !path.is_dir() {
            std::fs::create_dir_all(&path).unwrap();
        }

        let path = path.join(&self.sheet.name);

        let img_path = format!("{}.png", path.as_os_str().to_str().unwrap());
        self.image.save(img_path).unwrap();

        let json_path = format!("{}.json", path.as_os_str().to_str().unwrap());
        let json = serde_json::to_string_pretty(&self.sheet).unwrap();
        std::fs::write(json_path, json).unwrap();

        Ok(())
    }

    fn write_to_image(&mut self, x: u32, y: u32, img: &DynamicImage) {
        self.image.copy_from(img, x, y).unwrap();
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
    /// The x position of the frame in the sprite sheet
    pub x: u32,
    /// The y position of the frame in the sprite sheet
    pub y: u32,
    /// The width of the frame
    pub width: u32,
    /// The height of the frame
    pub height: u32,
}
