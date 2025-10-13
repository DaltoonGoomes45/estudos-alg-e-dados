mod indexer;
mod search;
mod recommender;

use indexer::ProductIndexer;
use search::SearchEngine;
use recommender::Recommender;
use std::io::{self, Write};

fn main() {
    let mut indexer = ProductIndexer::new();

    // Adicionando produtos
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

    let search_engine = SearchEngine::new(indexer.products.clone());
    let recommender = Recommender::new(indexer.products.clone());

    loop {
        println!("\n===== MegaStore Sistema de Busca =====");
        print!("Digite o nome do produto (ou 'sair' para encerrar): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("sair") {
            println!("Encerrando o sistema. Até logo!");
            break;
        }

        let resultados = search_engine.search_by_name(input);

        if resultados.is_empty() {
            println!("❌ Nenhum produto encontrado para '{}'", input);
        } else {
            println!("\n🔍 Produtos encontrados:");
            for produto in &resultados {
                println!(
                    "- {} (Categoria: {}, Marca: {}) [ID: {}]",
                    produto.name, produto.category, produto.brand, produto.id
                );
            }

            // Recomendações baseadas no primeiro resultado encontrado
            let recomendacoes = recommender.recommend(&resultados[0].id);
            if !recomendacoes.is_empty() {
                println!("\n💡 Recomendações:");
                for rec_id in recomendacoes {
                    if let Some(prod) = indexer.products.get(&rec_id) {
                        println!("- {} (Categoria: {}, Marca: {})", prod.name, prod.category, prod.brand);
                    }
                }
            } else {
                println!("💡 Nenhuma recomendação disponível.");
            }
        }
    }
}
