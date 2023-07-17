import os
import yaml

class Product:
    
    def __init__(self, path: str, number: str, name: str, color: list[int]):
        self.path = path
        self.number = number
        self.name = name
        self.surface_color = color
        
        
    def getPreviewPath(self):
        return self.path + "/preview.png"
        
        
    def get3dModelPath(self):
        return self.path + "/mesh.stl"
        
        
    def hasPreview(self):
        if os.path.exists(self.getPreviewPath()):
            return True
        return False
    
    
    def has3dModel(self):
        if os.path.exists(self.get3dModelPath()):
            return True
        return False


def readProduct(path: str):
    if not os.path.exists(path + "/product_info.yaml"):
        return None
    
    with open(path + "/product_info.yaml", "r") as yamlfile:
        data = yaml.load(yamlfile, Loader=yaml.FullLoader)
    
        art_num = data['number']
        name = data['name']
        surface_color = [int(data['surface_color']['red']), int(data['surface_color']['green']), int(data['surface_color']['blue'])]
    
        return Product(path, art_num, name, surface_color)
    
    
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
    
    return products
            

def debugProducts(products: list[Product]):
    for product in products:
        print(product.number + " " + product.name + " " + str(product.surface_color))
    
