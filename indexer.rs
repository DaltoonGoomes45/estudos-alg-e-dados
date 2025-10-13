use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
    pub brand: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProductIndexer {
    pub products: HashMap<String, Product>,
}

impl ProductIndexer {
    pub fn new() -> Self {
        Self { products: HashMap::new() }
    }

    pub fn add_product(&mut self, id: &str, name: &str, category: &str, brand: &str) {
        let product = Product {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            brand: brand.to_string(),
        };
        self.products.insert(id.to_string(), product);
    }
}
