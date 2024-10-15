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
ORTHO_SCALE = 5.0
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

#
# Helper functions
#

def set_camera(location, rotation, orthographic = True):
    for obj in bpy.context.scene.objects:
        if obj.type == 'CAMERA':
            # Set the position
            obj.location = location
            obj.rotation_euler = rotation
            
            if orthographic:
                # Convert camera to orthographic
                obj.data.type = 'ORTHO'
                obj.data.ortho_scale = ORTHO_SCALE
                obj.data.clip_start = 0.001
                obj.data.clip_end = 100.0


def set_lighting(rotation):
    LIGHTING_NAME = "__renderer_scene_lighting__"

    # remove any existing default lighting in the event we're using a previously rendered scene
    for obj in bpy.context.scene.objects:
        if obj.name == LIGHTING_NAME:
            obj.select_set(True)
            bpy.ops.object.delete()

    # create light datablock, set attributes
    light_data = bpy.data.lights.new(name=LIGHTING_NAME, type='SUN')
    light_data.energy = 1

    # create new object with our light datablock
    light_object = bpy.data.objects.new(
        name=LIGHTING_NAME, object_data=light_data)

    # link light object
    bpy.context.collection.objects.link(light_object)

    # make it active
    bpy.context.view_layer.objects.active = light_object

    # change props
    light_object.location = (0, 0, 10)
    light_object.rotation_euler = rotation

    # update scene, if needed
    dg = bpy.context.evaluated_depsgraph_get()
    dg.update()

def position(x, y, z):
    return Vector((x, y, z))

def rotation(x, y, z):
    rotation_rads = (math.radians(x), math.radians(y), math.radians(z))
    return Euler(rotation_rads)


# Triggers a render
def render(animation = "", perspective = ""):
    # Trigger render
    # Use '_ESCAPED' to prevent blender file names from mucking with the Rust parsing.
    bpy.context.scene.render.filepath = f'{CWD}/{OUTPUT_DIRECTORY}/{FILE_NAME}_{VIEW_TYPE}_ANIMATION-{animation}_PERSPECTIVE-{perspective}_FRAMENUMBER-'
    bpy.ops.render.render(animation=True, write_still=True)


#
# Render
#

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



# TODO:
# Need to render separate animations for each action
for action in bpy.data.actions:
    print(action)

# Render different perspectives
x = 2
z = 2
light_angle = 60
match VIEW_TYPE:
    case "InternalCamera":
        render(perspective="camera")
    case "Sidescroller":              
        set_lighting(rotation(0, light_angle, -180))
        set_camera(position(-x, 0, z), rotation(90, 0, -90))
        render(perspective="face-right")

        set_lighting(rotation(0, light_angle, 0))
        set_camera(position(x, 0, z), rotation(90, 0, 90))
        render(perspective="face-left")
    case "ThreeQuarter":
        print("ThreeQuarter")
    case "Isometric":
        print("Isometric")
    case "TopDown":        
        set_lighting(rotation(0, 0, 0))
        set_camera(position(0, 0, z), rotation(0, 0, -90))
        render(perspective="overhead")
    case "AdvanceWarsBattle":
        print("AdvanceWarsBattle")
    case "PokemonBattle":
        set_lighting(rotation(61, -17, 48))
        set_camera(position(4.6, -11.1, 3.2), rotation(80,0,-338))
        render(perspective="face")

        set_lighting(rotation(-36, 57, 62))
        set_camera(position(-3.4,11.2,3.7), rotation(78, 0, -162))
        render(perspective="back")
    case default:
        print("UNKNOWN VIEW_TYPE: " + VIEW_TYPE)
        sys.exit(1)
