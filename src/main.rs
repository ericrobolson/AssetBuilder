use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub enum Args {
    #[clap(about = "Generate a font map from a TTF file and text")]
    #[clap(
        long_about = "Takes in a TTF file, a string of text, and generates a fontmap_file file with the text rendered in the font. A JSON object detailing the spritesheet coordinates for each character will also be included."
    )]
    FontMap {
        /// Path to the TTF file
        ttf: PathBuf,
        /// String of text to render
        text: String,
        /// Path to output the fontmap_file file and json to
        fontmap_file: PathBuf,
    },
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args {
        Args::FontMap {
            ttf,
            text,
            fontmap_file,
        } => {
            generate_font_map(ttf, text, fontmap_file)?;
        }
    }

    Ok(())
}

fn generate_font_map(ttf: PathBuf, text: String, fontmap_file: PathBuf) -> Result<(), String> {
    // Validate the TTF file
    if !ttf.exists() {
        return Err(format!("TTF file does not exist: {:?}", ttf));
    }

    // Validate the TTF file is a TTF file
    if ttf.extension().unwrap().to_ascii_lowercase() != "ttf" {
        return Err(format!("TTF file is not a TTF file: {:?}", ttf));
    }

    // Validate the fontmap_file file ends with .fontmap_file
    if fontmap_file.extension().is_some() {
        return Err("FONTMAP_FILE should not have an extension".to_string());
    }

    Ok(())
}
