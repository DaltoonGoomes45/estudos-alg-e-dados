# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
## Descrição do Projeto

Este projeto implementa um sistema de busca e recomendação de produtos em Rust, utilizando HashMaps e grafos.
O objetivo é otimizar a busca de produtos no catálogo da empresa MegaStore, oferecendo resultados rápidos, precisos e relevantes, além de recomendações automáticas de produtos similares.

## Tecnologias Utilizadas

Linguagem: Rust

Crates: serde, serde_json, csv

Estruturas de dados: HashMap e lista de adjacência (grafo)

## Como Executar o Projeto

Instale o Rust:
https://www.rust-lang.org/tools/install

Compile o projeto:

cargo build


## Execute o sistema de busca:

cargo run


O sistema permite que você digite o nome do produto que deseja buscar.
Digite sair para encerrar o programa.

# Exemplo de execução:

Digite o nome do produto (ou 'sair' para encerrar): Notebook

Produtos encontrados:
- Notebook Lenovo (Categoria: Eletrônicos, Marca: Lenovo) [ID: 1]

Recomendações:
- Notebook Dell Inspiron (Categoria: Eletrônicos, Marca: Dell)
- Smartphone Samsung (Categoria: Eletrônicos, Marca: Samsung)

## Testes Automatizados

Para rodar os testes integrados:

cargo test


Os testes verificam:

Busca de produtos por nome

Recomendações baseadas na categoria

Indexação de todos os produtos cadastrados

⚠️ Importante:
Certifique-se de que o arquivo de testes (system_tests.rs) está dentro de uma pasta chamada tests/ na raiz do projeto.

# Exemplos de Produtos no Catálogo

Notebook Lenovo

Smartphone Samsung

Camisa Polo

Fone de Ouvido JBL

Smart TV LG

Tênis Nike

Cafeteira Nespresso

Liquidificador Arno

Sofá 3 lugares

Abajur Moderno

Pizza Congelada Sadia

Chocolate Lacta

Tênis Adidas

Jaqueta Columbia

Smartphone iPhone 14

Notebook Dell Inspiron

Fone Bluetooth Sony

Blusa de Frio Hering

Mesa de Jantar 6 lugares

Cerveja Heineken

## Arquitetura do Sistema
Arquivo	Função
indexer.rs	Indexa produtos usando HashMap
search.rs	Implementa os algoritmos de busca
recommender.rs	Gera recomendações baseadas em grafos
system_tests.rs	Testes automatizados do sistema

## Estruturas de Dados e Desempenho

HashMap: busca eficiente em tempo médio O(1)

Lista de adjacência (grafo): gera recomendações de produtos similares

Escalabilidade: suporta grandes catálogos mantendo desempenho estável
