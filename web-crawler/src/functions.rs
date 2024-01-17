use spider::website::Page;
use scraper::{Html, Selector};
use regex::Regex;

mod models;
pub use models::WebpageData;

fn parse_html(html_content: &str) -> String {
    // logic for parsing HTML content

    // transform into document
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("p").unwrap();
    let regex = Regex::new(r"[ \t\n]{2,}").unwrap();

    let mut content = String::new();

    for element in document.select(&selector) {
        let raw_text = element.text().collect::<String>();
        let clean_text = regex.replace_all(&raw_text, " ");
        content.push_str(&clean_text);
    }

    // return content as a string
    content

}

// define function to process data from pages
pub fn extract_webpage_data(page: &Page) -> WebpageData {

    // get link and html_content directly from page
    let link = page.get_url_final().to_string();
    let html_content = page.get_html();
    let parsed_content = parse_html(html_content);

    // return a struct of processed data 
    WebpageData {
        link: link,
        html_content: html_content, 
        parsed_content: parsed_content,
    }
}