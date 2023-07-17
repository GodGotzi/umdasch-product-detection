import numpy as np

import pyrender.scene as Scene
from pyrender.constants import DEFAULT_SCENE_SCALE

import pyrender
import trimesh
import matplotlib.pyplot as plt

import numpy as np

from scipy.spatial.transform import Rotation

import color as c
import random

def camera_pose(scene):
    centroid = scene.centroid

    scale = scene.scale
    if scale == 0.0:
        scale = DEFAULT_SCENE_SCALE

    s2 = 1.0 / np.sqrt(2.0)
    cp = np.eye(4)
    cp[:3,:3] = np.array([
        [0.0, -s2, s2],
        [1.0, 0.0, 0.0],
        [0.0, s2, s2]
    ])
    
    hfov = np.pi / 6.0
    dist = scale / (2.0 * np.tan(hfov)) * (random.random() * 0.8 + 0.8)

    cp[:3,3] = dist * np.array([1.0, 0.0, 1.0]) + centroid

    return cp

def rand_camera_pose(scene):
    centroid = scene.centroid

    scale = scene.scale
    if scale == 0.0:
        scale = DEFAULT_SCENE_SCALE

    s2 = 1.0 / np.sqrt(2.0)
    cp = np.eye(4)
    cp[:3,:3] = np.array([
        [0.0, -s2, s2],
        [1.0, 0.0, 0.0],
        [0.0, s2, s2]
    ])
    
    hfov = np.pi / 6.0
    dist = scale / (2.0 * np.tan(hfov)) * (random.random() * 0.8 + 0.8)

    cp[:3,3] = dist * np.array([1.0, 0.0, 1.0]) + centroid

    return cp

def random_rotation():
    rotation = [0, 0, 0]
    rotation[0] = int(random.random() * 360)
    rotation[1] = int(random.random() * 360)
    rotation[2] = int(random.random() * 360)
    
    return rotation
        

#scene = pyrender.Scene(ambient_light=[.1, .1, .7], bg_color=[255, 255, 255])
#camera = pyrender.PerspectiveCamera( yfov=np.pi / 3.0)
#light = pyrender.DirectionalLight(color=[1,1,1], intensity=10e2)
def generate_obj(source_path: str, destination_path: str, obj_color, scene: Scene, camera_node: pyrender.Node, random_perspective=False):
    #obj = trimesh.load("../products/mesh2.stl")
    obj = trimesh.load(source_path)

    #random_bg_color(obj_color)
    
    vertex_colors = [obj_color for _ in range(len(obj.vertices))]
    obj.visual.vertex_colors = vertex_colors
    mesh: pyrender.Mesh = pyrender.Mesh.from_trimesh(obj, smooth=False)
    
    # compose scene
    #scene: Scene = pyrender.Scene(ambient_light=[.1, .1, .7], bg_color=[255, 255, 255])
    #camera = pyrender.PerspectiveCamera( yfov=np.pi / 3.0)
    #light = pyrender.DirectionalLight(color=[1,1,1], intensity=10e2)
    #scene.add(light, pose=np.eye(4))
    #print(scene.centroid)
    if random_perspective:
        rotation = random_rotation()
        mp = np.eye(4)
        mp[:3,:3] = Rotation.from_euler('ZYX', rotation, degrees=True).as_matrix()
        
        mesh_node = scene.add(mesh, pose=mp)
        
        scene.set_pose(camera_node, rand_camera_pose(scene))
    else:
        mesh_node = scene.add(mesh)
        scene.set_pose(camera_node, camera_pose(scene))

    #scene.add(camera, pose=camera_pose(scene))
    
    r = pyrender.OffscreenRenderer(2048, 2048)
    color, _ = r.render(scene)

    plt.imsave(destination_path, color)
    r.delete()
    
    return mesh_node