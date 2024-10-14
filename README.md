# AssetBuilder

This repo is used for generating asses used in games. 

## Features
- `font-map` will take in a string of characters, a TTF font and a location. From there it will rasterize all characters to a spritesheet.
- - `font-map {TTF_FILE} {TEXT_TO_RENDER} {OUTPUT_DIR} [--font-scale {FLOAT}]

## Examples
Look at the `Makefile` to see a list of example invocations. 