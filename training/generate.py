import view as vp

import cv2

import color as color

import sys
import product as prod
import sample_detection as detection

def makePreview(product: prod.Product, camera_node, scene):
    scene.bg_color = [255, 255, 255]
    mesh_node = vp.generate_obj(product.get3dModelPath(), product.getPreviewPath(), product.surface_color, scene, camera_node, random_perspective=False)
    scene.remove_node(mesh_node)

if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("Usage: python generate.py <product_folder> <number_of_samples> <output_folder> <sample_info_file>")
        exit(1)

    products = prod.readProducts(sys.argv[1])
    prod.debugProducts(products)

    scene, camera_node = detection.build_scene()

    for product in products:
        if not product.has3dModel():
            print("WARNING Product has no 3d model: " + product.path)
            print("Skipping...")
            continue
        
        print("Generating samples for product: " + product.name)
        
        if not product.hasPreview():
            makePreview(product, camera_node, scene)
        
        for i in range(int(sys.argv[2])):
            print("Generating sample " + str(i) + "...")
            
            bg_color = color.random_bg_color(product.surface_color) 
            scene.bg_color = bg_color
            mesh_node = vp.generate_obj(product.get3dModelPath(), sys.argv[3] + "/" + product.name + "-" + str(i) + ".png", product.surface_color, scene, camera_node, random_perspective=True)

            img = cv2.imread(sys.argv[3] + "/" + product.name + "-" + str(i) + ".png")
            img_cpy = img.copy()

            gray_img = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
            #gray_img = cv2.GaussianBlur(gray_img, (5, 5), 0)

            thresh = cv2.adaptiveThreshold(gray_img, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2)

            contour_list, hierachy = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
            contour_list = list(contour_list)
            contour_list.pop(detection.largest_area(contour_list))

            countoured_img = img_cpy

            max_value_x, min_value_x = detection.boundaries_contour(contour_list, 0)
            max_value_y, min_value_y = detection.boundaries_contour(contour_list, 1)

            for i in range(len(contour_list)):
                countoured_img = cv2.drawContours(countoured_img, contour_list, i, (255, 0, 255), 3)

            cv2.rectangle(countoured_img, (min_value_x, min_value_y), (max_value_x, max_value_y), (0, 0, 255), 5)

            #countoured_img = cv2.drawContours(countoured_img, [max_contour], 0, (255, 0, 255), 3)

            cv2.imwrite(sys.argv[3] + "/" + product.name + "-" + str(i) + "-opencv-rect" + ".png", countoured_img)
            
            scene.remove_node(mesh_node)



