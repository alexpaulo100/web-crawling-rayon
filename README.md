# 🕸️ Web Crawling com Rayon (Rust)

Este projeto demonstra o uso de **paralelismo em Rust** com a biblioteca [`rayon`](https://docs.rs/rayon/latest/rayon/) para realizar web crawling na Wikipédia. 

O projeto tem como objetivo mostrar como podemos utilizar **threads** para acelerar a busca de páginas e processamento de conteúdo de maneira eficiente. A implementação utiliza a biblioteca **Rayon**, que simplifica o gerenciamento de múltiplas threads, permitindo o processamento paralelo de páginas web.

---

## 🚀 Tecnologias e Bibliotecas

- [`Rust`](https://www.rust-lang.org/) — linguagem principal
- [`rayon`](https://crates.io/crates/rayon) — paralelismo simples e performático
- [`wikipedia`](https://crates.io/crates/wikipedia) — API para acessar conteúdos da Wikipédia
- [`csv`](https://crates.io/crates/csv) — para geração de arquivos CSV
- [`regex`](https://crates.io/crates/regex) — extração da primeira frase com expressão regular

---

## 📄 O que o projeto faz?

- Acessa uma lista de jogadores brasileiros na Wikipédia.
- Coleta o conteúdo de cada página em paralelo.
- Extrai a **primeira frase** do texto.
- Conta a quantidade de palavras de cada artigo.
- Salva tudo em um arquivo CSV chamado `jogadores.csv`.

---

## 🏃 Como rodar o projeto?

### Pré-requisitos:

- Ter o Rust instalado: [https://rustup.rs](https://rustup.rs)

### Comandos:

```bash
# Clone o repositório
git clone https://github.com/alexpaulo100/web-crawling-rayon.git
cd web-crawling-rayon

# Execute o projeto
cargo run --release
