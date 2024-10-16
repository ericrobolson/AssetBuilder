run: FORCE
	cargo run -- help

test: FORCE
	cargo test

# Examples
## Font Map
font-map-help: example_dir
	cargo run -- font-map --help

font-map: example_dir
	cargo run -- font-map data/fonts/Open_Sans/OpenSans-Regular.ttf "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUFWXYZ0123456789,.?!@#$%^&()_+-"	example_results/font-map/atlas --font-scale 64.0

## Blend2Sheet
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

# TODO: Add examples for multiple animations; change to not use lighting

blend2sheet-anim-sidescroller: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 sidescroller

blend2sheet-anim-isometric: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 isometric --num-rotations 8

blend2sheet-anim-top-down: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 top-down

blend2sheet-anim-advance-wars-battle: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 32 32 advance-wars-battle

blend2sheet-anim-pokemon-battle: example_dir
	cargo run -- blend2sheet data/blender/Animated.blend example_results/blend2sheet/atlas 128 128 pokemon-battle


# Misc utilities
example_dir: FORCE
	rm -rdf example_results && mkdir example_results

FORCE:
