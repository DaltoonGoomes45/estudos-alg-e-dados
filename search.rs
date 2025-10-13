use std::collections::HashMap;
use crate::indexer::Product;

pub struct SearchEngine {
    products: HashMap<String, Product>,
}

impl SearchEngine {
    pub fn new(products: HashMap<String, Product>) -> Self {
        Self { products }
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Product> {
        self.products.values()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}
