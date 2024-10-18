## Misc eventually todos

- [ ] TODO: test on windows

## Unity asset 2 blender converter?

- Not sure if this is needed, but certainly would be nice

## 241017 Blender Spritesheet Generator

- [x] Build out new CLI option
- [x] Takes in a blender project name
- [x] Takes in an output directory
- [x] Takes in a sprite width
- [x] Takes in a view type
- [x] Add ability to list only certain animations for rendering
- [x] Call blender + run script
- [x] In script, set camera position based on view type. Add makefile entry
- [x] Get basic no animation original camera working
- [x] Setup views for each other view that doesn't have an animation
- [x] Implement rendering for animations for side scroller
- [x] Abstract sidescroller stuff to other views
- [x] Add ability to specify animations to use when rendering as CLI arg +implement in python
- [x] Get working on MacOS
- [x] Stitch together things into animations + spritesheets
- [x] Test out no animation spritesheets
- [x] Verify it still works on Linux
- [x] Convert it to densely pack spritesheets. E.g. for each frame, figure out what you can trim then update the offset stuff for that. May need to calculate the 'center' as well for the animation. Perhaps you always assume the sprite is drawn from (0,0), then an offset is included that tells you how much to add?

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
