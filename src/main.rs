mod font_map;
mod spritesheet;

use clap::Parser;
use std::path::PathBuf;

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

        /// The scale of the font. Default is 12.0.
        #[clap(long, default_value = "12.0")]
        font_scale: f32,
    },
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args {
        Args::FontMap {
            ttf,
            text,
            fontmap_file,
            font_scale,
        } => {
            font_map::run(ttf, text, fontmap_file, font_scale)?;
        }
    }

    Ok(())
}
