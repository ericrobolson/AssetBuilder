import bpy
import math
import os
import sys
from math import cos, sin, radians
from mathutils import Vector, Euler
CWD = os.getcwd()
FILE = bpy.path.basename(bpy.context.blend_data.filepath)  # Also the animation
FILE_NAME = FILE.replace(".blend", "")

# Some defaults
ORTHO_SCALE = 10.0
AA_SAMPLES = 16

# Parse args
argv = sys.argv
argv = argv[argv.index("--") + 1:]  # get all args after "--"

# CLI args
OUTPUT_DIRECTORY = argv[0]
RENDER_WIDTH = int(argv[1])
RENDER_HEIGHT = int(argv[2])
VIEW_TYPE = argv[3]
NUM_ROTATIONS = int(argv[4])
ANIMATIONS = argv[5]

def is_internal_camera():
    return VIEW_TYPE == "internal"

# Set the scene details to match what we want
for scene in bpy.data.scenes:
    scene.render.resolution_x = RENDER_WIDTH
    scene.render.resolution_y = RENDER_HEIGHT
    scene.render.resolution_percentage = 100
    scene.render.use_border = False
    scene.render.filter_size = 0.0    
    scene.render.film_transparent = True
    if scene.eevee:
        scene.eevee.taa_render_samples = AA_SAMPLES

# Need to render separate animations for each action
for action in bpy.data.actions:
    print(action)

# Print render engine
# print(f"Render engine: {bpy.context.scene.render.engine}")

# # Render if no actions
# if bpy.data.actions.len() == 0:
#     print("No actions, printing single frame")
#     bpy.context.scene.render.filepath = f'{CWD}/{OUTPUT_DIRECTORY}/{FILE_NAME}_ESCAPED_Static_ESCAPED'
#     print(f"Rendering {bpy.context.scene.render.filepath}")
#     bpy.ops.render.render(animation=True, write_still=True)
# else:
#     for action in bpy.data.actions:
#         print("Need to render " + action)

bpy.context.scene.render.filepath = f'{CWD}/{OUTPUT_DIRECTORY}/{FILE_NAME}_ESCAPED_Static_ESCAPED'
print(f"Rendering {bpy.context.scene.render.filepath}")
bpy.ops.render.render(animation=True, write_still=True)


# def render(perspective):
#     # Trigger render
#     # Use '_ESCAPED' to prevent blender file names from mucking with the Rust parsing.
#     bpy.context.scene.render.filepath = f'{CWD}/{OUTPUT_DIRECTORY}/{FILE_NAME}_ESCAPED{perspective}_ESCAPED'
#     print(f"Rendering {bpy.context.scene.render.filepath}")
#     bpy.ops.render.render(animation=True, write_still=True)





# render(123)

