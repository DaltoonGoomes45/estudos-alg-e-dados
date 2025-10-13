# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este projeto implementa um sistema de busca e recomendação de produtos em Rust, utilizando **HashMaps** e **grafos**.  
O objetivo é otimizar a busca de produtos no catálogo da empresa MegaStore, oferecendo resultados rápidos, precisos e relevantes, além de recomendações automáticas de produtos similares.

---

## Tecnologias Utilizadas
- Linguagem: **Rust**
- Crates: `serde`, `serde_json`, `csv`
- Estruturas de dados: **HashMap** e **lista de adjacência (grafo)**

---

## Como Executar
1. Instale o Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
2. Compile o projeto:
```bash
cargo build
Execute o sistema de busca:

bash
Copiar código
cargo run
O sistema permite que você digite o nome do produto que deseja buscar. Digite sair para encerrar.

Exemplo:

less
Copiar código
Digite o nome do produto (ou 'sair' para encerrar): Notebook
🔍 Produtos encontrados:
- Notebook Lenovo (Categoria: Eletrônicos, Marca: Lenovo) [ID: 1]

💡 Recomendações:
- Notebook Dell Inspiron (Categoria: Eletrônicos, Marca: Dell)
- Smartphone Samsung (Categoria: Eletrônicos, Marca: Samsung)
Testes
Para executar os testes automatizados:

bash
Copiar código
cargo test
O projeto possui testes unitários cobrindo:

Busca de produtos por nome

Recomendações baseadas na categoria do produto

Todos os produtos adicionados no índice

Exemplos de Produtos Adicionados
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

Arquitetura
indexer.rs → Indexa produtos usando HashMap

search.rs → Implementa algoritmos de busca

recommender.rs → Gera recomendações usando grafos

Estruturas de Dados e Desempenho
HashMap: busca eficiente em O(1)

Lista de adjacência (grafo): recomendações de produtos similares

Escalável para catálogos grandes, mantendo performance consistente