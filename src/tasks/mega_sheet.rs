use crate::spritesheet_gen::{render_animations, stitch_together_renders, ViewType};
use std::path::PathBuf;

pub fn run(
    source_directory: PathBuf,
    output_directory: PathBuf,
    output_name: String,
    sprite_width: u32,
    sprite_height: u32,
    view_type: ViewType,
    num_rotations: u32,
) -> Result<(), String> {
    // Use
    // https://github.com/ericrobolson/BuilderGenerator
    // as reference

    validate(
        &source_directory,
        &output_directory,
        &output_name,
        sprite_width,
        sprite_height,
        num_rotations,
    )?;

    //
    // Render all frames and animations
    //

    // Using walkdir, recursively search for all .blend files in the source directory
    let blender_render_dir = output_directory.join(".blender_render");
    for entry in walkdir::WalkDir::new(&source_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if entry.path().extension().unwrap() == "blend" {
            let blender_file = entry.path();

            std::fs::create_dir_all(&output_directory).unwrap();
            std::fs::create_dir_all(&blender_render_dir).unwrap();

            let script_path = PathBuf::from("data/render_blender.py");

            render_animations(
                blender_file.to_path_buf(),
                script_path.clone(),
                sprite_width,
                sprite_height,
                view_type.clone(),
                num_rotations,
                String::default(),
                blender_render_dir.clone(),
            )?;
        }
    }

    //
    // Now stitch together the sprite sheet
    //

    stitch_together_renders(
        &blender_render_dir,
        &output_directory,
        crate::spritesheet_gen::AnimationNaming::Custom(output_name),
    )?;

    Ok(())
}

fn validate(
    source_directory: &PathBuf,
    output_directory: &PathBuf,
    output_name: &str,
    sprite_width: u32,
    sprite_height: u32,
    num_rotations: u32,
) -> Result<(), String> {
    if output_name.is_empty() {
        return Err("Output name must not be empty".to_string());
    }

    // Validate blender_file
    if !source_directory.exists() {
        return Err(format!(
            "Source directory {:?} does not exist",
            source_directory
        ));
    }
    if !source_directory.is_dir() {
        return Err(format!(
            "Source directory {:?} is not a directory",
            source_directory
        ));
    }

    // Validate output_directory
    if output_directory.exists() && !output_directory.is_dir() {
        return Err(format!(
            "Output directory {:?} is not a directory",
            output_directory
        ));
    }

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
