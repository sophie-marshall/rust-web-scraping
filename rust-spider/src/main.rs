// import relevant structs 
use spider::tokio; // use for asynchronous tasks 
use  spider::website::{Website, self};
// use scraper::Html;
mod functions;
// use functions::{get_content};
// use std::collections::HashMap;
// use std::hash::Hash;
// use std::thread::current;
// use std::time::{Instant, Duration};
// use regex::Regex;
// use polars::prelude::*;
// use polars::df;
// use reqwest;

// enable asynchronous runtimes 
#[tokio::main]
// define async main function
async fn main() {

    // INSTANTIATE REQUIRED STRUCTS // 
    let mut queue: Vec<String> = Vec::new();
    // let mut visited: Vec<String> = Vec::new();
    // let mut content_hash: HashMap<String, String> = HashMap::new();

    let root_url = "https://help.pbs.org/";
    
    queue.push(root_url.to_string());
    queue.push("https://help.pbs.org/support/solutions/articles/12000059662-how-to-use-the-pbs-app-for-samsung-smart-tv-".to_string());
    // queue.push("url3".to_string());
    // queue.push("url4".to_string());

    // BEGIN SCRAPING LOOP // 
    while let Some(element) = queue.pop() {

        println!("Current URL: {}", &element);

        // let mut website = Website::new(&element);
        // website.crawl().await;

        // for link in website.get_links() {
        //     queue.push(link.as_ref().to_string());
        // }

        println!("New Queue Length: {}", queue.len());

    }

    println!("Vector is empty!")

}
