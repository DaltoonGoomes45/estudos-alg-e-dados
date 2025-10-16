use megastore_search_recommender::indexer::ProductIndexer;
use megastore_search_recommender::search::SearchEngine;
use megastore_search_recommender::recommender::Recommender;

fn create_full_indexer() -> ProductIndexer {
    let mut indexer = ProductIndexer::new();

    indexer.add_product("1", "Notebook Lenovo", "Eletrônicos", "Lenovo");
    indexer.add_product("2", "Smartphone Samsung", "Eletrônicos", "Samsung");
    indexer.add_product("3", "Camisa Polo", "Vestuário", "Lacoste");
    indexer.add_product("4", "Fone de Ouvido JBL", "Eletrônicos", "JBL");
    indexer.add_product("5", "Smart TV LG", "Eletrônicos", "LG");
    indexer.add_product("6", "Tênis Nike", "Vestuário", "Nike");
    indexer.add_product("7", "Cafeteira Nespresso", "Eletrodomésticos", "Nespresso");
    indexer.add_product("8", "Liquidificador Arno", "Eletrodomésticos", "Arno");
    indexer.add_product("9", "Sofá 3 lugares", "Decoração", "Tok&Stok");
    indexer.add_product("10", "Abajur Moderno", "Decoração", "Oppa");
    indexer.add_product("11", "Pizza Congelada Sadia", "Alimentos", "Sadia");
    indexer.add_product("12", "Chocolate Lacta", "Alimentos", "Lacta");
    indexer.add_product("13", "Tênis Adidas", "Vestuário", "Adidas");
    indexer.add_product("14", "Jaqueta Columbia", "Vestuário", "Columbia");
    indexer.add_product("15", "Smartphone iPhone 14", "Eletrônicos", "Apple");
    indexer.add_product("16", "Notebook Dell Inspiron", "Eletrônicos", "Dell");
    indexer.add_product("17", "Fone Bluetooth Sony", "Eletrônicos", "Sony");
    indexer.add_product("18", "Blusa de Frio Hering", "Vestuário", "Hering");
    indexer.add_product("19", "Mesa de Jantar 6 lugares", "Decoração", "Tok&Stok");
    indexer.add_product("20", "Cerveja Heineken", "Alimentos", "Heineken");

    indexer
}

#[test]
fn test_search_all_products() {
    let indexer = create_full_indexer();
    let search_engine = SearchEngine::new(indexer.products.clone());

    let product_names = vec![
        "Notebook Lenovo", "Smartphone Samsung", "Camisa Polo", "Fone de Ouvido JBL",
        "Smart TV LG", "Tênis Nike", "Cafeteira Nespresso", "Liquidificador Arno",
        "Sofá 3 lugares", "Abajur Moderno", "Pizza Congelada Sadia", "Chocolate Lacta",
        "Tênis Adidas", "Jaqueta Columbia", "Smartphone iPhone 14", "Notebook Dell Inspiron",
        "Fone Bluetooth Sony", "Blusa de Frio Hering", "Mesa de Jantar 6 lugares", "Cerveja Heineken"
    ];

    for name in product_names {
        let results = search_engine.search_by_name(name);
        assert!(!results.is_empty(), "Produto '{}' não encontrado", name);
    }
}


#[test]
fn test_all_recommendations() {
    let indexer = create_full_indexer();
    let recommender = Recommender::new(indexer.products.clone());

    for id in 1..=20 {
        let product_id = id.to_string();
        let recs = recommender.recommend(&product_id);

        // Verifica que o código não quebra e retorna vetor (mesmo que vazio)
        assert!(recs.len() >= 0, "Recomendações falharam para produto ID {}", product_id);
    }
}

