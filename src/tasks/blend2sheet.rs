use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
    view_type: ViewType,
    num_rotations: u32,
) -> Result<(), String> {
    validate(
        &blender_file,
        &output_directory,
        sprite_width,
        num_rotations,
    )?;

    Ok(())
}

fn validate(
    blender_file: &PathBuf,
    output_directory: &PathBuf,
    sprite_width: u32,
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

    // Validate num_rotations is None or greater than 0
    if num_rotations == 0 {
        return Err("Number of rotations must be greater than 0".to_string());
    }

    Ok(())
}
