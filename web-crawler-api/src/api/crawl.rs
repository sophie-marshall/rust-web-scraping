use actix_web::{web, Responder};
use actix_web::{post, HttpResponse};
use crate::{models::response::Response, models::input::Input};
use serde::{Serialize, Deserialize};

use spider::website::Website;
use spider::page::Page;
use regex::Regex;
use scraper::{Html, Selector};

// define struct
// #[derive(Serialize)]
// struct LinkList {
//     links: Vec<String>,
// }

// define different storage struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebpageData {
    pub link: String,
    pub parsed_content: String,
}

// define response object 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrawledDataResponse {
    pub crawled_data: Vec<WebpageData>,
}

// helper functions
fn parse_html(html_content: &str) -> String {
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
    let parsed_content = parse_html(&html_content);

    // return a struct of processed data 
    WebpageData {
        link: link, 
        parsed_content: parsed_content,
    }
}

// set crawling endpoint 
#[post("/crawl")]
pub async fn crawl_url(input: web::Json<Input>) -> impl Responder {
    // get base_url to crawl from Web 
    let base_url = input.message.to_string();

    // instantiate website object
    let mut website = Website::new(&base_url);

    // configure website object
    website
        .with_respect_robots_txt(true)
        .with_subdomains(false) 
        .with_tld(false) 
        .with_delay(1000) // SM: switch delay for production?
        .with_request_timeout(None)
        .with_http2_prior_knowledge(false)
        .with_user_agent(Some("innovation team scraping experiment bot v1.0").into())
        .with_on_link_find_callback(Some(|link, html| {
            println!("Link: {}", link.inner());
            (link, html)
        })) // SM: adjust to increrment counter?
        .with_headers(None)
        .with_blacklist_url(None) 
        .with_proxies(None); // SM: may be worth setting up later to help distribute traffic?

    // begin scrape
    website.scrape().await;

    // instantiate storage struct
    let mut crawled_data: Vec<WebpageData> = Vec::new();

    // iterate over pages, grab content, and store it
    if let Some(pages) = website.get_pages() {

        for page in pages.iter() {
            let data = extract_webpage_data(page);
            crawled_data.push(data);

        }
    } else {
        eprintln!("Failed to retrieve website content")
    }

    

    // // get links 
    // let links: Vec<String> = website
    //     .get_links()
    //     .iter()
    //     .map(|link| link.as_ref().to_string())
    //     .collect();

    // let num_links = links.len();

    // put into struct 
    // let link_list = LinkList { links };

    // convert 
    // let link_response = serde_json::to_string(&link_list).unwrap();

    // let response = Response {
    //     message: num_links.to_string(),
    // };

    let response = CrawledDataResponse {
        crawled_data,
    };

    let json_response = serde_json::to_string(&response);

    match json_response {
        Ok(json) => HttpResponse::Ok().json(json),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

    // match serde_json::to_string(&response) {
    //     Ok(json_response) => HttpResponse::Ok()
    //         .content_type("application/json")
    //         .body(json_response),
    //     Err(e) => {
    //         eprintln!("Failed to serialize response to JSON: {}", e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }

}

