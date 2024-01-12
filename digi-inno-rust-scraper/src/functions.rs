use scraper::{Html, Selector};
use regex::Regex;
use spider::website::Website;
use spider::tokio;

#[tokio::main]
pub async fn get_links(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Instantiate link vector
    let mut links: Vec<String> = Vec::new();

    let mut website = Website::new(url);
    website.crawl_smart().await;

    for link in website.get_links() {
        links.push(link.as_ref().to_string());
    }

    Ok(links)
}

// function to get text from a url
pub fn get_content(url: &str) -> String {

    let response = reqwest::blocking::get(url);
    let html_content = response.unwrap().text().unwrap();
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("p").unwrap();
    let regex = Regex::new(r"[ \t\n]{2,}").unwrap();

    let mut content = String::new();

    for element in document.select(&selector) {

        let raw_text = element.text().collect::<String>();
        let clean_text = regex.replace_all(&raw_text, " ");

        content.push_str(&clean_text);
    }

    // return content
    content
}