use std::error::Error;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use regex::Regex;
use spider::website::Website;
use spider::tokio;

// function to crawl a webpage for associated links 
#[tokio::main]
pub async fn get_links(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // instantiate link vector
    let mut links: Vec<String> = Vec::new();
    // instantiate website object for crawling
    let mut website = Website::new(url);
    website.crawl_smart().await;
    // execute smart crawl 
    for link in website.get_links() {
        links.push(link.as_ref().to_string());
    }
    // return link provided crawl worked
    Ok(links)
}

// function to grab paragraph content form a webpage
pub fn get_content(url: &str) -> String {
    // initiate reqwest client to handle behavior
    let client = Client::builder()
                            .redirect(reqwest::redirect::Policy::limited(5))
                            .build()
                            .unwrap();
    // make request 
    let response = client.get(url).send();
    // check and handle response types
    match response {
        Ok(response) => {
            // if request okay parse and return content
            let html_content = response.text().unwrap();
            let document = Html::parse_document(&html_content);

            let selector = Selector::parse("p").unwrap();
            let regex = Regex::new(r"[ \t\n]{2,}").unwrap();

            let mut content = String::new();

            for element in document.select(&selector) {
                let raw_text = element.text().collect::<String>();
                let clean_text = regex.replace_all(&raw_text, " ");
                content.push_str(&clean_text);
            }
            // return content if things worked
            content
        }
        // if error, return error types
        Err(err) => {
            // If there was an error, check if it's a TooManyRedirects error
            if let Some(reqwest_error) = err.source() {
                if reqwest_error.is::<reqwest::Error>() {
                    // Handle the redirect error by returning a custom message
                    "redirect limit reachted".to_string()
                } else {
                    // If it's not a redirect error, return the original error
                    format!("Error: {}", err)
                }
            } else {
                // If there's no source, return the original error
                format!("Error: {}", err)
            }
        }
    }
}