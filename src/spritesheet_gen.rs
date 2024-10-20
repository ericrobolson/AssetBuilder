use clap::{Parser, ValueEnum};
use core::panic;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use crate::spritesheet::SpriteSheetBuilder;

/// The type of view the sprite sheet will be generated from
#[derive(Parser, ValueEnum, Clone, Debug)]
pub enum ViewType {
    /// Classic platformer view.
    Sidescroller,
    /// Isometric view like Diablo.
    Isometric,
    /// Top down view like in Hotline Miami.
    TopDown,
    /// A special view for Advance Wars style games. Based off the battle cutscenes.
    AdvanceWarsBattle,
    /// A special view for Pokemon style games. Based off the battle cutscenes.
    PokemonBattle,
    /// Use the internal blender camera and all its settings.
    InternalCamera,
}

pub fn get_blender_location() -> String {
    // List different potential paths this can be, then iterate over them to find the proper one.

    let mut options = vec![
        "blender".to_string(),
        "/Applications/Blender.app/Contents/MacOS/Blender".to_string(),
    ];

    if let Ok(output) = Command::new("which").arg("blender").output() {
        let output = std::str::from_utf8(&output.stdout).unwrap();
        options.push(output.trim().to_string());
    }

    // Now iterate over them and see which one exists
    for option in options {
        if let Ok(output) = Command::new(&option).arg("--version").output() {
            let output = std::str::from_utf8(&output.stdout).unwrap();
            if output.is_empty() == false {
                return option;
            }
        }
    }

    panic!("Could not find blender executable; error 4004b");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationNaming {
    /// Naming convention for a single object in a spritesheet
    SingleObject,
    /// Naming convention for multiple objects in the same spritesheet
    MultiObject,
}

/// Stitch together all renders in a directory into a single sprite sheet
pub fn stitch_together_renders(
    blender_render_dir: &PathBuf,
    output_dir: &PathBuf,
    animation_naming: AnimationNaming,
) -> Result<(), String> {
    if !blender_render_dir.exists() {
        return Err(format!(
            "Blender render directory does not exist: {:?}",
            blender_render_dir
        ));
    }

    if !blender_render_dir.is_dir() {
        return Err(format!(
            "Blender render directory is not a directory: {:?}",
            blender_render_dir
        ));
    }

    if !output_dir.is_dir() {
        return Err(format!(
            "Output directory is not a directory: {:?}",
            output_dir
        ));
    }

    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }

    // List out all .png files in the output directory
    let files = std::fs::read_dir(&blender_render_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .filter(|path| path.extension().unwrap() == "png")
        .collect::<Vec<_>>();

    // Helper function to extract the value of a key from a file name
    let find_value = |key: &str, contents: &str| -> String {
        let idx = contents.find(&key).expect(&format!(
            "Could not find '{}-name]' in file {}",
            key, contents
        ));
        let value = &contents[idx..];
        let value = value.split("]").collect::<Vec<&str>>()[0];
        value.replace(&key, "").trim().to_string()
    };

    // For each rendered file, extract the view type, file, animation, and perspective
    // then add it to the animations hashmap
    let mut animations: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut name: Option<String> = None;
    for path in files {
        let s = path.to_str().unwrap();
        let view_type = find_value("[VIEWTYPE-", s);
        let file = find_value("[FILE-", s);
        let animation = find_value("[ANIMATION-", s);
        let perspective = find_value("[PERSPECTIVE-", s);

        name = Some(file.clone());

        let key = format!("{}.{}.{}.{}", file, view_type, animation, perspective);

        if !animations.contains_key(&key) {
            animations.insert(key.clone(), Vec::new());
        }

        animations.get_mut(&key).unwrap().push(path.clone());
    }

    // Sort every animation so everything is properly ordered
    for (_, v) in animations.iter_mut() {
        v.sort();
    }

    // Now that we have all animations, let's load them and determine the size of the sprite sheet

    let mut animation_images = HashMap::new();
    for (animation, frames) in animations.iter() {
        let mut images = vec![];

        for frame in frames {
            let img = image::open(frame).unwrap();

            images.push(img);
        }

        animation_images.insert(animation.clone(), images);
    }

    // Iterate over all images and add them to the sprite sheet
    let name = match animation_naming {
        AnimationNaming::SingleObject => name.unwrap_or_default(),
        AnimationNaming::MultiObject => "MegaSheet".to_string(),
    };
    let mut sprite_sheet = SpriteSheetBuilder::new(name);

    // Now for every animation image, add it to the sprite sheet
    for (animation, imgs) in animation_images.iter() {
        for frame in imgs.iter() {
            sprite_sheet.add_sprite(animation.clone(), frame.clone());
        }
    }

    // Save the sprite sheet
    sprite_sheet.save(output_dir)?;

    Ok(())
}

/// Render all animations in a blender file to the given directory
pub fn render_animations(
    blender_file: PathBuf,
    script_path: PathBuf,
    sprite_width: u32,
    sprite_height: u32,
    view_type: ViewType,
    num_rotations: u32,
    animations: String,
    blender_render_dir: PathBuf,
) -> Result<(), String> {
    let blender_exe = get_blender_location();
    let command_output = Command::new(blender_exe)
        .arg("-b")
        .arg(blender_file.clone())
        // Load a python script
        .arg("-P")
        .arg(script_path)
        // Debug events
        // .arg("--debug-python")
        // Add in some args for the Python script
        .arg("--")
        .arg(&blender_render_dir)
        .arg(sprite_width.to_string())
        .arg(sprite_height.to_string())
        .arg(format!("{:?}", view_type))
        .arg(num_rotations.to_string())
        .arg(animations)
        // Execute
        .output();

    match command_output {
        Ok(e) => {
            let output = std::str::from_utf8(&e.stdout).unwrap();
            println!("{}", output);
        }
        Err(e) => return Err(format!("{:?}", e)),
    }

    Ok(())
}
