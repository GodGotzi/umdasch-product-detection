import os
import shutil

object_path = 'STL'
id = 0
products: list[str] = []

for file in os.listdir(object_path):
    #os.mkdir(os.path.splitext(file)[0])
    #shutil.copyfile('STL/' + file, os.path.splitext(file)[0] + "/mesh.stl")
    
    #yaml_conf = open(os.path.splitext(file)[0] + "/product_info.yaml", "w+")
    #number = os.path.splitext(file)[0]
    products.append(os.path.splitext(file)[0])
    #yaml_conf.write(f"number: {number}\nname: {number}\nclass_id: {id}\nsurface_color: \n  red: 5\n  blue: 5\n  green: 5")
    #yaml_conf.close()
    id+=1
    
print(products)
print(len(products))

