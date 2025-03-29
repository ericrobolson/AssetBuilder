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
        #[clap(long)]
        ttf: PathBuf,
        /// Path to where text files are located
        #[clap(long)]
        text_files_dir: PathBuf,
        /// File extension to use when sourcing text
        #[clap(long)]
        text_file_extension: String,
        /// Path to output the fontmap_file file and json to
        #[clap(long)]
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
        /// The name of the output sprite sheet
        output_name: String,
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
    #[clap(name = "resize-imgs", about = "Resize images in a directory")]
    ResizeImgs {
        /// Path to the directory of images
        source_directory: PathBuf,
        /// The amount to scale each image by
        scale: f32,
    },
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    match args {
        Args::FontMap {
            ttf,
            text_files_dir,
            text_file_extension,
            fontmap_directory,
            font_scale,
        } => {
            tasks::font_map::run(
                ttf,
                text_files_dir,
                text_file_extension,
                fontmap_directory,
                font_scale,
            )?;
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
            output_name,
            sprite_width,
            sprite_height,
            view_type,
            num_rotations,
        } => {
            tasks::mega_sheet::run(
                source_directory,
                output_directory,
                output_name,
                sprite_width,
                sprite_height,
                view_type,
                num_rotations,
            )?;
        }
        Args::ResizeImgs {
            source_directory,
            scale,
        } => {
            tasks::resize_images::run(source_directory, scale)?;
        }
    }

    Ok(())
}
