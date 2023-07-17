import view as vp
import os

import cv2

import color as color

import numpy as np
import pyrender

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
        print(max_value)
        print(min_value)
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

scene, camera_node = build_scene()

obj_color = [255, 255, 0]
bg_color = color.random_bg_color(obj_color) 
scene.bg_color = bg_color
vp.generate_obj("../products/mesh2.stl", "save.png", obj_color, scene, camera_node)

img = cv2.imread("save.png")
img_cpy = img.copy()

gray_img = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
#gray_img = cv2.GaussianBlur(gray_img, (5, 5), 0)

thresh = cv2.adaptiveThreshold(gray_img, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2)

contour_list, hierachy = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
contour_list = list(contour_list)
contour_list.pop(largest_area(contour_list))

countoured_img = img_cpy

max_value_x, min_value_x = boundaries_contour(contour_list, 0)
max_value_y, min_value_y = boundaries_contour(contour_list, 1)

for i in range(len(contour_list)):
    countoured_img = cv2.drawContours(countoured_img, contour_list, i, (255, 0, 255), 3)

cv2.rectangle(countoured_img, (min_value_x, min_value_y), (max_value_x, max_value_y), (0, 0, 255), 5)

#countoured_img = cv2.drawContours(countoured_img, [max_contour], 0, (255, 0, 255), 3)


cv2.imwrite("opencv_save.png", countoured_img)
#cv2.imshow('Input Img', countoured_img)
cv2.waitKey(0)
cv2.destroyAllWindows()



