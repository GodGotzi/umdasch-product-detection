import os
import yaml

class Product:
    
    def __init__(self, path: str, number: str, name: str, color: list[int], class_id: int):
        self.path = path
        self.number = number
        self.name = name
        self.surface_color = color
        self.class_id = class_id
        
        
    def getPreviewPath(self, fext: str):
        return self.path + "/preview" + fext
        
        
    def get3dModelPath(self, meshfext: str):
        return self.path + "/mesh" + meshfext
        
        
    def hasPreview(self, fext: str):
        if os.path.exists(self.getPreviewPath(fext)):
            return True
        return False
    
    
    def nextFreePath(self, output_folder: str, last_index: int, fext: str):
        i = last_index
        while True:
            path = output_folder + "/" + self.name + "-" + str(i) + fext
            if not os.path.exists(path):
                return path, i
            i += 1
    
    
    def has3dModel(self, meshfext: str):
        if os.path.exists(self.get3dModelPath(meshfext)):
            return True
        return False


def readProduct(path: str):
    if not os.path.exists(path + "/product_info.yaml"):
        return None
    
    with open(path + "/product_info.yaml", "r") as yamlfile:
        data = yaml.load(yamlfile, Loader=yaml.FullLoader)
    
        if data is None:
            return None
    
        if not 'number' in data or not 'name' in data or not 'surface_color' in data or not 'class_id' in data:
            return None
    
        art_num = str(data['number'])
        name = str(data['name'])
        class_id = int(data['class_id'])
        
        if not 'red' in data['surface_color'] or not 'green' in data['surface_color'] or not 'blue' in data['surface_color']:
            return None
        
        surface_color = [int(data['surface_color']['red']), int(data['surface_color']['green']), int(data['surface_color']['blue'])]
    
        return Product(path, art_num, name, surface_color, class_id)
    
    
def readProducts(path: str):
    products = []
    
    folders = os.listdir(path)
    
    for folder in folders:
        if os.path.isdir(path + "/" + folder):
            product = readProduct(path + "/" + folder)
            if product is not None:
                products.append(product)
            else:
                print("WARNING Product not found in folder: " + folder)
                continue
    
    return products
            

def debugProducts(products: list[Product]):
    for product in products:
        print(product.number + " " + product.name + " " + str(product.surface_color))
    
