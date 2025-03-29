run: FORCE
	cargo run -- help

test: FORCE
	cargo test

# Examples
## Font Map
font-map-help: example_dir
	cargo run -- font-map --help

font-map: example_dir
	cargo run -- font-map --ttf data/fonts/Open_Sans/OpenSans-Regular.ttf --text-files-dir data/fonts/ --text-file-extension txt	--fontmap-directory example_results/font-map/atlas --font-scale 64.0


font-map-pixel: example_dir
	cargo run -- font-map --ttf data/fonts/Open_Sans/OpenSans-Regular.ttf --text-files-dir data/fonts/ --text-file-extension txt	--fontmap-directory example_results/font-map/atlas --font-scale 24.0

## Resize images
resize-imgs-help: example_dir
	cargo run -- resize-imgs --help
 
resize-imgs: example_dir
	cargo run -- resize-imgs data/example_resize-imgs 0.25

## Blend2Sheetblend2sheet-anim-sidescroller
### No animations
blend2sheet-no-anim-internal-camera: example_dir
	cargo run -- blend2sheet data/blender/Suzanne.blend example_results/blend2sheet/atlas 320 240 internal-camera

blend2sheet-no-anim-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/SuzanneNoLighting.blend example_results/blend2sheet/atlas 128 128 sidescroller

blend2sheet-no-anim-isometric: example_dir
	cargo run -- blend2sheet data/blender/SuzanneNoLighting.blend example_results/blend2sheet/atlas 128 128 isometric --num-rotations 8

blend2sheet-no-anim-top-down: example_dir
	cargo run -- blend2sheet data/blender/SuzanneNoLighting.blend example_results/blend2sheet/atlas 128 128 top-down

blend2sheet-no-anim-advance-wars-battle: example_dir
	cargo run -- blend2sheet data/blender/SuzanneNoLighting.blend example_results/blend2sheet/atlas 32 32 advance-wars-battle

blend2sheet-no-anim-pokemon-battle: example_dir
	cargo run -- blend2sheet data/blender/SuzanneNoLighting.blend example_results/blend2sheet/atlas 128 128 pokemon-battle

### Multiple animations
blend2sheet-anim-internal-camera: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 320 240 internal-camera

blend2sheet-anim-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 sidescroller

blend2sheet-anim-isometric: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 256 256 isometric --num-rotations 8

blend2sheet-anim-top-down: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 top-down

blend2sheet-anim-advance-wars-battle: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 32 32 advance-wars-battle

blend2sheet-anim-pokemon-battle: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 pokemon-battle

### Specified animations

blend2sheet-two-anims-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 sidescroller --animations "Walk,Idle"

blend2sheet-one-anims-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 sidescroller --animations "Walk"

blend2sheet-no-anims-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 sidescroller --animations "DOESNTEXIST"

## MegaSheet
mega-sheet-help: example_dir
	cargo run -- mega-sheet --help

mega-sheet: example_dir
	cargo run -- mega-sheet data/blender/ example_results/mega-sheet/atlas MegaSheetName 128 128 sidescroller

# Misc utilities
example_dir: FORCE
	rm -rdf example_results && mkdir example_results

FORCE:
