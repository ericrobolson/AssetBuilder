# AssetBuilder

This repo is used for generating assests used in games. 

## Features
- `font-map` will take in a string of characters, a TTF font and a location. From there it will rasterize all characters to a spritesheet.
- - `font-map {TTF_FILE} {TEXT_TO_RENDER} {OUTPUT_DIR} [--font-scale {FLOAT}]
- `blend2sheet` will take in a Blender file, a view type, and generate a spritesheet based on the animations included if they exist. The resulting file will be a power of 2 to minimize GPU hiccups.
- - `blend2sheet {BLENDER_FILE} {OUTPUT_DIR} {SPRITE_WIDTH} {SPRITE_HEIGHT} [sidescroller isometric top-down advance-wars-battle pokemon-battle camera] [--num-rotations {INT}] [--animations {CSV_ANIMATION_LIST}]`
- - - `sidescroller` view type renders the sprite in a platformer view
- - - `isometric` view type renders the sprite in an isometric view
- - - `top-down` view type renders the sprite in a top down view
- - - `advance-wars-battle` view type renders the sprite in a side view seen in Advance Wars battles
- - - `pokemon-battle` view type renders the sprite in two views: a front and back view. This is what is typically seen during Pokemon Fire Red for example.
- - - `internal-camera` view type simply uses the Blender camera and all its settings.
- - - `--num-rotations INT` is an optional argument that is only applicable for isometric views. 
- - - `--animations CSV_ANIMATIONS_LIST` is an optional argument that takes a CSV list of animations. When provided it will only render those animations.
- - - If there are errors, try deleting all default lights as well as any default cameras.

## Examples
Look at the `Makefile` to see a list of example invocations. 