use clap::{Parser, ValueEnum};
use std::fs::DirEntry;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// The type of view the sprite sheet will be generated from
#[derive(Parser, ValueEnum, Clone, Debug)]
pub enum ViewType {
    /// Classic platformer view.
    Sidescroller,
    /// 3/4 view. Example would be the game boy Legend of Zeldas.
    #[clap(name = "3/4")]
    ThreeQuarter,
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

pub fn run(
    blender_file: PathBuf,
    output_directory: PathBuf,
    sprite_width: u32,
    sprite_height: u32,
    view_type: ViewType,
    num_rotations: u32,
    animations: String,
) -> Result<(), String> {
    // Use
    // https://github.com/ericrobolson/BuilderGenerator
    // as reference

    validate(
        &blender_file,
        &output_directory,
        sprite_width,
        sprite_height,
        num_rotations,
    )?;

    let script_path = PathBuf::from("data/render_blender.py");

    let command_output = Command::new("blender")
        .arg("-b")
        .arg(blender_file.clone())
        // Load a python script
        .arg("-P")
        .arg(script_path)
        // Debug events
        // .arg("--debug-python")
        // Add in some args for the Python script
        .arg("--")
        .arg(output_directory)
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
            Ok(())
        }
        Err(e) => Err(format!(
            "Error rendering blend file {:?}: {:?}",
            blender_file, e
        )),
    }
}

fn validate(
    blender_file: &PathBuf,
    output_directory: &PathBuf,
    sprite_width: u32,
    sprite_height: u32,
    num_rotations: u32,
) -> Result<(), String> {
    // Validate blender_file
    if !blender_file.exists() {
        return Err(format!("Blender file {:?} does not exist", blender_file));
    }
    if !blender_file.is_file() {
        return Err(format!("Blender file {:?} is not a file", blender_file));
    }
    if !blender_file.extension().unwrap_or_default().eq("blend") {
        return Err(format!(
            "Blender file {:?} is not a .blend file",
            blender_file
        ));
    }

    // Validate output_directory
    if output_directory.exists() && !output_directory.is_dir() {
        return Err(format!(
            "Output directory {:?} is not a directory",
            output_directory
        ));
    }
    std::fs::create_dir_all(&output_directory).unwrap();

    // Validate sprite_width is greater than 0
    if sprite_width == 0 {
        return Err("Sprite width must be greater than 0".to_string());
    }

    // Validate sprite_height is greater than 0
    if sprite_height == 0 {
        return Err("Sprite height must be greater than 0".to_string());
    }

    // Validate num_rotations is None or greater than 0
    if num_rotations == 0 {
        return Err("Number of rotations must be greater than 0".to_string());
    }

    Ok(())
}
