run: FORCE
	cargo run -- help

test: FORCE
	cargo test

exampe_dir: FORCE
	rm -rdf example_results && mkdir example_results

font-map: exampe_dir
	cargo run -- font-map data/fonts/Open_Sans/OpenSans-Regular.ttf "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUFWXYZ0123456789,.?!@#$%^&()_+-"	example_results/font-map/atlas --font-scale 64.0

font-map-help: exampe_dir
	cargo run -- font-map --help

FORCE:
