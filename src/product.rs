use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Clone)]
pub struct ProductServer {
    product_folder: String,
    products: Vec<Product>,
}

impl ProductServer {

    pub fn new(product_folder: String) -> Self {
        Self { product_folder: product_folder, products: vec![] }
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let paths = fs::read_dir(self.product_folder.as_str())?;

        for path in paths {
            let path = path?;
            if path.path().is_dir() {
                let file_path = format!("{}/product_info.yaml", path.path().to_str().unwrap());
                let str = fs::read_to_string(file_path.as_str())?;

                let mut product: Product = serde_yaml::from_str(&str).unwrap();
                product.path = Some(file_path);
                self.products.push(product);
            }
        }
        
        self.products.sort_by(|p1, p2| {
            if p1.class_id < p2.class_id {
                return Ordering::Less;
            } else if p1.class_id > p2.class_id {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });

        Ok(())
    }

    pub fn get_by_id(&self, id: usize) -> &Product {
        &self.products[id]
    }

    pub fn len(&self) -> usize {
        self.products.len()
    }

    pub fn products(&self) -> &Vec<Product> {
        &self.products
    }

}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(skip_serializing)]
    pub path: Option<String>,

    pub number: String,
    pub name: String,
    pub class_id: u32,
    pub surface_color: HashMap<String, u8>,
}

/*
number: 7098285
name: 7098285
class_id: 57
surface_color: 
  red: 5
  blue: 5
  green: 5
*/