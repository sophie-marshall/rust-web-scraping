use reqwest;
use scraper::{Html, Selector};

// define public structures
#[derive(Debug)]
pub struct WebpageContent {
    pub url: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub links: Vec<String>,
}

// define error types
#[derive(Debug)]
pub enum WebpageError {
    RequestError(reqwest::Error),

}

// function to load webpage content and associated infromation 
pub fn get_webpage_content(url: &str) -> Result<WebpageContent, WebpageError> {

    let response = reqwest::blocking::get(url).map_err(WebpageError::RequestError)?;
    let html_content = response.text().map_err(WebpageError::RequestError)?;

    let document = Html::parse_document(&html_content);

    // extract necessary data from document 
    let url = Some(url.to_owned());

    let title = document
        .select(&Selector::parse("h1").unwrap())
        .next()
        .map(|h1| h1.text().collect::<String>());

    let content = document
        .select(&Selector::parse("div#main-content.wiki-content").unwrap())
        .next()
        .map(|element| element.text().collect::<String>());

    let links: Vec<String> = document
        .select(&Selector::parse("a").unwrap())
        .filter_map(|a| a.value().attr("href").map(str::to_owned))
        .filter(|link| link.contains("https"))
        .collect();

    // create WebpageContent structure
    Ok(WebpageContent {
        url, 
        title, 
        content, 
        links
    })

}