import multiprocessing
import os
import view as vp

import cv2

import color as color

import sys
import product as prod
import sample_detection as detection
import structure_raw as structraw

def makePreview(product: prod.Product, camera_node, scene):
    scene.bg_color = [255, 255, 255]
    mesh_node = vp.generate_obj(product.get3dModelPath(sys.argv[4]), product.getPreviewPath(sys.argv[3]), product.surface_color, scene, camera_node, random_perspective=False)
    scene.remove_node(mesh_node)
    
    
def createAnnotationFile(class_id: int, path: str, x, y, w, h):
    f = open(path, "w+")
    
    w = float(w)/float(sys.argv[8])
    h = float(h)/float(sys.argv[8])
    
    x_s = float(x)/float(sys.argv[8])
    y_s = float(y)/float(sys.argv[8])
    
    annotation: str = str(class_id) + " " + str(x_s + 0.5 * w) + " " + str(y_s + 0.5 * h) + " " + str(w) + " " + str(h)
    f.write(annotation)
    f.close()
    
    
def handleSample(sample_index: int):
    global current_product
    global offset
    
    print("Product: " + current_product.name + " | Generating sample " + str(sample_index) + "...")
    scene, camera_node = detection.build_scene()
    
    bg_color = color.random_bg_color(current_product.surface_color) 
    scene.bg_color = bg_color
    
    path = sys.argv[7] + "/raw/" + current_product.name + "-" + str(sample_index + offset)
    
    if len(sys.argv) == 10:
        opencv_path = sys.argv[9] + "/" + current_product.name + "-" + str(sample_index + offset) + "-opencv-rect"
    else:
        opencv_path = None
    
    mesh_node = vp.generate_obj(current_product.get3dModelPath(sys.argv[4]), path + sys.argv[3], current_product.surface_color, scene, camera_node, random_perspective=True)

    img: cv2.Mat = cv2.imread(path + sys.argv[3])

    gray_img = cv2.cvtColor(img.copy(), cv2.COLOR_BGR2GRAY)
    #gray_img = cv2.GaussianBlur(gray_img, (5, 5), 0)

    thresh = cv2.adaptiveThreshold(gray_img, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 2)

    contour_list, hierachy = cv2.findContours(thresh, cv2.RETR_TREE, cv2.CHAIN_APPROX_SIMPLE)
    contour_list = list(contour_list)
    contour_list.pop(detection.largest_area(contour_list))

    max_value_x, min_value_x = detection.boundaries_contour(contour_list, 0)
    max_value_y, min_value_y = detection.boundaries_contour(contour_list, 1)

    for i in range(len(contour_list)):
        countoured_img = cv2.drawContours(img, contour_list, i, (255, 0, 255), 3)

    cv2.rectangle(countoured_img, (min_value_x, min_value_y), (max_value_x, max_value_y), (0, 0, 255), 5)

    if opencv_path is not None:
        cv2.imwrite(opencv_path + sys.argv[3], countoured_img)
        
    createAnnotationFile(current_product.class_id, path + ".txt", min_value_x, min_value_y, max_value_x-min_value_x, max_value_y-min_value_y)
    
    scene.remove_node(mesh_node)
    
def init_worker(product: prod.Product, _offset: int):
    # declare scope of a new global variable
    global current_product
    # store argument in the global variable for this process
    current_product = product
    
    global offset
    offset = _offset

if __name__ == "__main__":
    # python generate.py --manual [products] <number_of_samples> <output_folder> <sample_info_file>
    
    opencv_save_path: str = None
    
    if len(sys.argv) != 10 and len(sys.argv) != 9:
        print("python generate.py --auto/--manual --recreate/--create <picture_file_extension> <mesh_file_extension> <product_folder>/[products] <number_of_samples> <output_folder> <img-size> <opencv-rect-output>")
        exit(1)
    
    if sys.argv[1] == "--manual":
        product_paths = sys.argv[5].split(",")
        products = []
        
        for path in product_paths:
            print("Reading product: " + path)
            product: prod.Product = prod.readProduct(path)
            if product is None:
                print("WARNING Product could not be read: " + path)
                print("Skipping...")
                continue
            products.append(prod.readProduct(path))
            
    elif sys.argv[1] == "--auto":
        products = prod.readProducts(sys.argv[5])
    else:
        print("python generate.py --auto/--manual --recreate/--create <picture_file_extension> <mesh_file_extension> <product_folder>/[products] <number_of_samples> <output_folder> <img-size> <opencv-rect-output>")
        exit(1)     
        
    if not os.path.exists(sys.argv[7]):
        os.mkdir(sys.argv[7])
        
    if len(sys.argv) == 10:
        if not os.path.exists(sys.argv[9]):
            os.mkdir(sys.argv[9])
    
    prod.debugProducts(products)
    
    if sys.argv[2] == "--recreate":
        recreate = True
    elif sys.argv[2] == "--create":
        recreate = False
    else:
        print("python generate.py --auto/--manual --recreate/--create <picture_file_extension> <mesh_file_extension> <product_folder>/[products] <number_of_samples> <output_folder> <img-size> <opencv-rect-output>")
        exit(1)

    sample_size = int(sys.argv[6])

    for product in products:
        scene, camera_node = detection.build_scene()
        
        if not product.has3dModel(sys.argv[4]):
            print("WARNING Product has no 3d model: " + product.path)
            print("Skipping...")
            continue
        
        print("Generating samples for product: " + product.name)
        
        if not product.hasPreview(sys.argv[3]):
            makePreview(product, camera_node, scene)
        
        scene.clear()
        
        _, offset = product.nextFreePath(sys.argv[7], 0, sys.argv[3])
        
        if recreate:
            offset = 0
        
        pool = multiprocessing.Pool(initializer=init_worker, initargs=(product, offset))
        pool.map(handleSample, range(sample_size))
        pool.close()
    
    structraw.structurize(sys.argv[3], sys.argv[7], sys.argv[7] + "/raw")