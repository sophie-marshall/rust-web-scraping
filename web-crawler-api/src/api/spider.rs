use crate::api::spider_config::configure_crawler;
use crate::api::spider_functions::extract_webpage_data;
use crate::models::input::Input;
use crate::models::crawled_data_response::CrawledDataResponse;
use crate::models::webpage_data::WebpageData;

use actix_web::{web, Responder};
use actix_web::{post, HttpResponse};

#[post("/crawl")]
pub async fn crawl_url(input: web::Json<Input>) -> impl Responder {
    // take base url from webpage 
    let base_url = input.message.to_string();
    // create webpage object with config
    let mut website = configure_crawler(&base_url);
    // scrape webpage 
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
    // prepare response 
    let response = CrawledDataResponse{ crawled_data };
    let json_response = serde_json::to_string(&response);
    // send HTTP response 
    match json_response {
        // if okay, send the json back to requester
        Ok(json) => HttpResponse::Ok().json(json),
        // else send server error
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}