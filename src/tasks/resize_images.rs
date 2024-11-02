use std::path::PathBuf;

use image::GenericImageView;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "bmp", "tga"];

pub fn run(source_directory: PathBuf, scale: f32) -> Result<(), String> {
    // Verify that scale is above zero
    if scale <= 0.0 {
        return Err("Scale must be greater than zero".to_string());
    }

    // Ensure path is a directory
    if !source_directory.exists() {
        return Err("Source directory does not exist".to_string());
    }
    if !source_directory.is_dir() {
        return Err("Source directory must be a directory".to_string());
    }

    let mut paths = vec![];
    for entry in walkdir::WalkDir::new(&source_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let ext = entry.path().extension().unwrap();

        if EXTENSIONS.contains(&ext.to_str().unwrap()) {
            paths.push(entry);
        }
    }

    //
    // Resize images
    paths.par_iter().for_each(|entry| {
        println!("Resizing {:?}", entry.path());
        let img = image::open(entry.path())
            .map_err(|e| e.to_string())
            .unwrap();

        let (width, height) = img.dimensions();
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        let resized = img.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        resized.save(entry.path()).unwrap();
    });

    Ok(())
}
