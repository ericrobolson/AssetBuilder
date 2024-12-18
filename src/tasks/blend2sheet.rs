use crate::spritesheet_gen::{render_animations, stitch_together_renders, ViewType};
use std::path::PathBuf;

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

    //
    // Render all frames and animations
    //

    let blender_render_dir = output_directory.join(".blender_render");

    std::fs::create_dir_all(&output_directory).unwrap();
    std::fs::create_dir_all(&blender_render_dir).unwrap();

    let script_path = PathBuf::from("data/render_blender.py");
    let current_dur = std::env::current_dir().unwrap();

    let blender_file = current_dur.join(blender_file);
    let script_path = current_dur.join(script_path);

    render_animations(
        blender_file,
        script_path.clone(),
        sprite_width,
        sprite_height,
        view_type.clone(),
        num_rotations,
        animations.clone(),
        blender_render_dir.clone(),
    )?;

    //
    // Now stitch together the sprite sheet
    //

    stitch_together_renders(
        &blender_render_dir,
        &output_directory,
        crate::spritesheet_gen::AnimationNaming::SingleObject,
    )?;

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
