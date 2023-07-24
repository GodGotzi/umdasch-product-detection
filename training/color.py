import random

def random_bg_color(obj_color):
    bg_color = [0, 0, 0]
    
    rand = random.random()
    while is_close_color(obj_color, bg_color):
        bg_color[0] = int(rand * 255.0 + 125.0)
        bg_color[1] = int(rand * 255.0 + 125.0)
        bg_color[2] = int(rand * 255.0 + 125.0)
    
    return bg_color


def is_close_color(color1, color2):
    color1 = [color1[0] + 1, color1[1] + 1, color1[2] + 1]
    color2 = [color2[0] + 1, color2[1] + 1, color2[2] + 1]
    
    if color1[0] / color2[0] > 0.8 and color1[1] / color2[1] > 0.8 and color1[2] / color2[2] > 0.8:
        return True
    
    #if (color1[0] / color1[1] / color1[2]) / (color2[0] / color2[1] / color2[2]) > 0.8:
    #    return True
    
    return False

def is_same_color(color1, color2):
    if color1[0] == color2[0] and color1[1] == color2[1] and color1[2] == color2[2]:
        return True
    return False    