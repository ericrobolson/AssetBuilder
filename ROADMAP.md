## Blender Spritesheet Generator
- [x] Build out new CLI option
- [x] Takes in a blender project name
- [x] Takes in an output directory
- [x] Takes in a sprite width
- [x] Takes in a view type 
- [x] Add ability to list only certain animations for rendering
- [ ] Call blender + run script
- [ ] In script, set camera position based on view type. Add makefile entry 
- [ ] For each view type, set up a blend file and makefile script to run.
- [ ] Render basic sprite without any animations (call it TPose) and do it for each animation.
- [ ] Do this for all view types/rotations

## 24-10-14 Font bitmap sheet
- Takes in text and a font file, then generates a spritesheet with coordinates to each character. 
- [x] Add in a new CLI option to the main program
- [x] Specify a font file
- [x] Specify a file or folder for reading text files
- [x] Add loading of font, then rasterizing an image and saving to disk.
- [x] Convert it to do this for each character.
- [x] Build a spritesheet for this. Use characters to index the regions to use for the sprite.
- [x] Write each character to a spritesheet and add its coordinates to an object that maps characters to coordinates
- [x] Save everything to two files, a spritesheet of all characters and a json object that contains the mapping of characters to regions on the spritesheet