use clap::{Parser, ValueEnum};
use core::panic;
use std::path::PathBuf;
use std::process::Command;

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
    let current_dur = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dur);
    let blender_file = current_dur.join(blender_file);
    let script_path = current_dur.join(script_path);
    println!("script: {:?}", script_path);

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
        }
        Err(e) => return Err(format!("{:?}", e)),
    }

    // Now stitch together the sprite sheet
    // TODO:
    Ok(())
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
