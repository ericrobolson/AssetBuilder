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


blend2sheet-sidescroller: example_dir
	cargo run -- blend2sheet data/blend2sheet/blend2sheet.blend example_results/blend2sheet/atlas 32 sidescroller

# TODO: add one for each view type

# Misc utilities
example_dir: FORCE
	rm -rdf example_results && mkdir example_results

FORCE:
