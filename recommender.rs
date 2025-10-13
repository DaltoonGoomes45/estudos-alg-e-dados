use std::collections::HashMap;
use crate::indexer::Product;

#[allow(dead_code)]
pub struct Recommender {
    graph: HashMap<String, Vec<String>>,
    products: HashMap<String, Product>,
}

impl Recommender {
    pub fn new(products: HashMap<String, Product>) -> Self {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for (id1, p1) in &products {
            for (id2, p2) in &products {
                if id1 != id2 && p1.category == p2.category {
                    graph.entry(id1.clone()).or_default().push(id2.clone());
                }
            }
        }

        Self { graph, products }
    }

    pub fn recommend(&self, product_id: &str) -> Vec<String> {
        self.graph.get(product_id).cloned().unwrap_or_default()
    }
}
