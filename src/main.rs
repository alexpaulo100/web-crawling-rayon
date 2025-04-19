use csv::Writer;
use rayon::prelude::*;
use regex::Regex;
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;

#[derive(Clone)]
struct ProcessedPage {
    title: String,
    data: String,
}

const PAGES: [&str; 10] = [
    "Pelé",
    "Ademir da Guia",
    "Ronaldo",
    "Ronaldinho",
    "Neymar",
    "Rivaldo",
    "Romário",
    "Roberto Carlos",
    "Falcão",
    "Bebeto",
];

fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page
        .get_title()
        .unwrap_or_else(|_| "Título não encontrado".to_string());
    let content = page
        .get_content()
        .unwrap_or_else(|_| "Conteúdo não encontrado".to_string());

    ProcessedPage {
        title,
        data: content,
    }
}

fn main() {
    let start = std::time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();

    let pages: Vec<_> = PAGES
        .iter()
        .map(|p| wikipedia.page_from_title(p.to_string()))
        .collect();

    let processed_pages: Vec<_> = pages.par_iter().map(process_page).collect();

    for page in &processed_pages {
        let start_page = std::time::Instant::now();
        println!("Title: {}", page.title);

        //let first_sentence = page.data.split('.').next().unwrap_or("");
        let first_sentence = get_first_sentence(&page.data);

        println!("First sentence: {}", first_sentence);

        let word_count = page.data.split_whitespace().count();
        println!("Word count: {}", word_count);

        println!("Page time: {:?}", start_page.elapsed());
        println!("-----------------------------");
    }

    println!("Total time: {:?}", start.elapsed());
    println!("Number of threads: {}", rayon::current_num_threads());

    save_to_csv(processed_pages.clone());
}

fn get_first_sentence(content: &str) -> String {
    let re = Regex::new(r"(?s)^\s*.*?[.!?](?:\s|$)").unwrap();
    match re.find(content) {
        Some(matched) => matched.as_str().to_string(),
        None => content.to_string(),
    }
}

fn save_to_csv(processed_pages: Vec<ProcessedPage>) {
    let mut wtr = Writer::from_path("jogadores.csv").unwrap();

    // Cabeçalhos do CSV
    wtr.write_record(["Title", "First Sentence", "Word Count"])
        .unwrap();

    for page in processed_pages {
        wtr.write_record(&[
            page.title,
            get_first_sentence(&page.data),
            page.data.split_whitespace().count().to_string(),
        ])
        .unwrap();
    }

    wtr.flush().unwrap();
}
