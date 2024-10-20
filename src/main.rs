mod spritesheet;
mod spritesheet_gen;
mod tasks;

use clap::Parser;
use spritesheet_gen::ViewType;
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
        fontmap_directory: PathBuf,
        /// The scale of the font. Default is 12.0.
        #[clap(long, default_value = "12.0")]
        font_scale: f32,
    },
    #[clap(about = "Generate a sprite sheet from a Blender file")]
    #[clap(
        long_about = "Takes in a Blender file, an output directory, the sprite width and view type."
    )]
    #[clap(name = "blend2sheet")]
    Blend2Sheet {
        /// Path to the Blender file
        blender_file: PathBuf,
        /// Path to output the sprite sheet to
        output_directory: PathBuf,
        /// The width of each sprite in the sheet
        sprite_width: u32,
        /// The height of each sprite in the sheet
        sprite_height: u32,
        /// The type of view the sprite sheet will be generated from
        view_type: ViewType,
        /// The number of rotations to generate for each sprite. Only used on 3/4 and isometric views.
        #[clap(long, required = false, default_value = "8")]
        num_rotations: u32,
        /// A comma separated list of animations to generate. If empty, all animations will be generated.
        #[clap(long, required = false, default_value = "")]
        animations: String,
    },
    #[clap(about = "Generate a mega sprite sheet from a directory of images")]
    #[clap(
        long_about = "Takes in a directory of blender files, an output directory, the sprite size and view type. Will render each file into a single sprite sheet."
    )]
    #[clap(name = "mega-sheet")]
    MegaSheet {
        /// Path to the directory of blender files
        source_directory: PathBuf,
        /// Path to output the sprite sheet to
        output_directory: PathBuf,
        /// The width of each sprite in the sheet
        sprite_width: u32,
        /// The height of each sprite in the sheet
        sprite_height: u32,
        /// The type of view the sprite sheet will be generated from
        view_type: ViewType,
        /// The number of rotations to generate for each sprite. Only used on 3/4 and isometric views.
        #[clap(long, required = false, default_value = "8")]
        num_rotations: u32,
    },
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args {
        Args::FontMap {
            ttf,
            text,
            fontmap_directory,
            font_scale,
        } => {
            tasks::font_map::run(ttf, text, fontmap_directory, font_scale)?;
        }
        Args::Blend2Sheet {
            blender_file,
            output_directory,
            sprite_width,
            view_type,
            num_rotations,
            animations,
            sprite_height,
        } => {
            tasks::blend2sheet::run(
                blender_file,
                output_directory,
                sprite_width,
                sprite_height,
                view_type,
                num_rotations,
                animations,
            )?;
        }
        Args::MegaSheet {
            source_directory,
            output_directory,
            sprite_width,
            sprite_height,
            view_type,
            num_rotations,
        } => {
            tasks::mega_sheet::run(
                source_directory,
                output_directory,
                sprite_width,
                sprite_height,
                view_type,
                num_rotations,
            )?;
        }
    }

    Ok(())
}
