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
ORTHO_SCALE = 5
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


LIGHTING_NAME = "__renderer_scene_lighting__"

def get_light():

    # remove any existing default lighting in the event we're using a previously rendered scene
    for obj in bpy.context.scene.objects:
        if obj.name == LIGHTING_NAME:
            return obj
    return None

def skip_action_setting(obj):
    if obj is None:
        return True
    if obj.type == 'CAMERA':
        return True
    if obj.name == LIGHTING_NAME:
        return True
    return False

def set_camera(location, rotation, orthographic = True):
    CAMERA_NAME = "__renderer_scene_camera__"

    # Remove existing cameras
    for obj in bpy.data.objects:
        if obj.type == 'CAMERA':
            obj.select_set(True)
            bpy.ops.object.delete()

    # Create new camera
    camera_data = bpy.data.cameras.new(name=CAMERA_NAME)
    camera = bpy.data.objects.new(name=CAMERA_NAME, object_data=camera_data)
    bpy.context.scene.collection.objects.link(camera)
    bpy.context.scene.camera = camera

    # make it active
    bpy.context.view_layer.objects.active = camera

    # change props
    print(location)
    camera.location = location
    camera.rotation_euler = rotation

    if orthographic:
        # Convert camera to orthographic
        camera.data.type = 'ORTHO'
        camera.data.ortho_scale = ORTHO_SCALE
        camera.data.clip_start = 0.001
        camera.data.clip_end = 100.0

def set_lighting(rotation):
    LIGHTING_NAME = "__renderer_scene_lighting__"

    # remove any existing default lighting in the event we're using a previously rendered scene
    for obj in bpy.data.objects:
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


def rotate_obj(obj):
    if obj is None:
        return
    euler = Euler((0, 0, radians(45)), 'XYZ')
    obj.rotation_euler.rotate(euler)


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


# Render different perspectives
x = 2
z = 2
light_angle = 60


def render_sidescroller(animation = ""):
    set_lighting(rotation(0, 60, -180))
    set_camera(position(-2, 0, 2), rotation(90, 0, -90))
    render(perspective="face-right")

    set_lighting(rotation(0, 60, 0))
    set_camera(position(2, 0, 2), rotation(90, 0, 90))
    render(perspective="face-left", animation=animation)

def undo_animation(obj):
    if obj is None:
        return
    if obj.animation_data is None:
        return
    if obj.animation_data and obj.animation_data.nla_tracks:
        for nt in obj.animation_data.nla_tracks:
            obj.animation_data.nla_tracks.remove(nt)
    if obj.animation_data and obj.animation_data.drivers:
        for dr in obj.animation_data.drivers:
                obj.animation_data.drivers.remove(dr)
    obj.animation_data.action = None


def render_sidescroller():
    perspectives = [
        {
            "light_rotation": rotation(0, 60, -180),
            "camera_position": position(-2, 0, 2),
            "camera_rotation": rotation(90, 0, -90),
            "perspective": "face-right"
        },
        {
            "light_rotation": rotation(0, 60, 0),
            "camera_position": position(2, 0, 2),
            "camera_rotation": rotation(90, 0, 90),
            "perspective": "face-left"
        }
    ]
    perform_render(perspectives)

def perform_render(perspectives):
    # Do static renders
    if not bpy.data.actions:
        for perspective in perspectives:
            set_lighting(perspective["light_rotation"])
            set_camera(perspective["camera_position"], perspective["camera_rotation"])
            render(perspective=perspective["perspective"])


    # Do animations
    for action in bpy.data.actions:
        # Set frame range
        animation_name = action.name

        bpy.context.scene.frame_start = int(action.frame_start)
        bpy.context.scene.frame_end = int(action.frame_end)

        for scene in bpy.data.scenes:
            scene.frame_start = int(action.frame_start)
            scene.frame_end = int(action.frame_end)

        # Set action for armatures
        for obj in bpy.data.objects:
            if obj.animation_data and obj.type == 'ARMATURE':
                obj.animation_data.action = action

        # Render perspectives
        for perspective in perspectives:
            set_lighting(perspective["light_rotation"])
            set_camera(perspective["camera_position"], perspective["camera_rotation"])
            render(perspective=perspective["perspective"], animation=animation_name)

        # Reset
        for obj in bpy.data.objects:
            undo_animation(obj)

match VIEW_TYPE:

    case "InternalCamera":
        render(perspective="camera")
    case "Sidescroller":
        render_sidescroller()
    case "Isometric":
        degs_per_rotation = 360.0 / float(NUM_ROTATIONS)
        initial_rotation = 0

        # default position + rotation
        init_x = 0
        init_y = -15
        z = 10
        pos = position(init_x, init_y, z)
        rot = rotation(60, 0, initial_rotation)

        for rotation_idx in range(0, NUM_ROTATIONS):
            rot_degs = -degs_per_rotation * rotation_idx
            rot_rads = radians(rot_degs)

            # Get new position + rotation for camera and light
            new_x = init_x * cos(rot_rads) + init_y * sin(rot_rads)
            new_y = -init_x * sin(rot_rads) + init_y * cos(rot_rads)

            pos = position(new_x, new_y, z)

            # Leave this
            rot = rotation(60, 0, -rot_degs)
            light_rot = rotation(40, 0, -rot_degs)

            set_lighting(light_rot)
            set_camera(pos, rot)

            render(perspective=f'rotation-{rotation_idx}')
    case "TopDown":        
        set_lighting(rotation(0, 0, 0))
        set_camera(position(0, 0, z), rotation(0, 0, -90))
        render(perspective="overhead")
    case "AdvanceWarsBattle":
        set_lighting(rotation(71,-9.6,64.5))
        set_camera(position(14.56,-8.29,6.2), rotation(75,0,-300))
        render(perspective="face-left")

        set_lighting(rotation(71,7,-55.1))
        set_camera(position(-36.15,-20.78, 12.962), rotation(75,0,-420))
        render(perspective="face-right")
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
