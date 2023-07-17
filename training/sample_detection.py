import pyrender

import cv2

import numpy as np

def build_scene():
    scene = pyrender.Scene(ambient_light=[.1, .1, .7], bg_color=[255, 255, 255])
    camera = pyrender.PerspectiveCamera( yfov=np.pi / 3.0)
    light = pyrender.DirectionalLight(color=[1,1,1], intensity=10e2)

    scene.add(light, pose=np.eye(4))
    node = scene.add(camera, pose=np.eye(4))
    
    return scene, node
    
def boundaries_contour(contour_list, index):
    max_value = 0
    min_value = 2048
    
    for contour in contour_list:
        for p in contour:
            if max_value < p[0][index]:
                max_value = p[0][index]
            
            if min_value > p[0][index]:
                min_value = p[0][index]
    
    return max_value, min_value


def largest_area(contour_list):
    max_area_index = None
    
    for i in range(len(contour_list)):
        if not max_area_index is None:
            if cv2.contourArea(contour_list[max_area_index]) < cv2.contourArea(contour_list[i]):
                max_area_index = i                
        else:
            max_area_index = i    
    
    return max_area_index