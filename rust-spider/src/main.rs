// import relevant structs 
use  spider::website::Website;
use spider::tokio; // use for asynchronous tasks 
use scraper::Html;
mod functions;
use functions::{get_content};
use std::collections::HashMap;
use std::time::{Instant, Duration};
// use regex::Regex;
// use polars::prelude::*;
// use polars::df;
// use reqwest;

// enable asynchronous runtimes 
#[tokio::main]
// define async main function
async fn main() {

    // INSTANTIATE REQUIRED STRUCTS // 
    let mut links_to_scrape: Vec<String> = Vec::new();
    let mut links_visited: Vec<String> = Vec::new();
    let mut link_content_hash: HashMap<String, String> = HashMap::new();

    // SET BASE URL //
    let start_url = "https://help.pbs.org/";

    // BEGIN SCRAPING LOOP // 

    // add base url to links_to_scrape is full
    links_to_scrape.push(start_url.to_string());

    let max_duration = Duration::from_secs(30);
    let mut counter = 0;
    let start_time = Instant::now();

    // continue while we are still able to find a link in scraping queue
    while let Some(element) = links_to_scrape.pop() {
        
        while Instant::now().duration_since(start_time) < max_duration {

            // define website struct 
            let mut website = Website::new(&element);
    
            // crawl website
            website.scrape().await;
    
            // for each page found ... 
            for page in website.get_pages().unwrap().iter().take(1) {
    
                // get page content
                let document = Html::parse_document(&page.get_html());
                let content = get_content(&document);
    
                // // add current data to hashmap 
                // link_content_hash.insert(page.get_url_final().to_string(), content);
    
                // // add url to links visited
                // links_visited.push(page.get_url_final().to_string()); //SM: GET URL FINAL IS JUST RETURNING THE STARTING URL -- FIX THIS 
    
                // search for new urls on current page 
                let mut website = Website::new(&page.get_url_final().to_string());
                website.crawl_smart().await;
    
                // append unvisited and unqueued links to queue
                for new_link in website.get_links() {
                    let str_link = new_link.as_ref().to_string();
                    if !links_to_scrape.contains(&str_link) && !links_visited.contains(&str_link) {
                        links_to_scrape.push(new_link.as_ref().to_string());
                    }
                }
    
            }
    
        }
    }

    println!("Timer Up:");

    for link in &links_visited {
        println!("{}", link);
    }


    // println!("Vector is empty!")

    

    // // define mutable website struct 
    // let mut website = Website::new(&start_url);

    // // crawl webstie
    // website.scrape().await;

    // create empty hashmap 

    // // for each page found on the website...
    // for page in website.get_pages().unwrap().iter().take(1) {

    //     // get page content
    //     let document = Html::parse_document(&page.get_html()); 
    //     let content = get_content(&document);
        
    //     // add current content and link to hashmap 
    //     link_content_hash.insert(page.get_url_final().to_string(), content);
        
    //     // add the url to links visited
    //     links_visited.push(page.get_url_final().to_string());
        
    //     // crawl the current page for new urls
    //     // if not in visited, appedend to the new urls list
    //     let mut website = Website::new(&page.get_url_final());
    //     website.crawl_smart().await;

    //     for new_link in website.get_links(){
    //         // if link not in queue and hasnt been visited...
    //         let str_link = new_link.as_ref().to_string();
    //         if !links_to_scrape.contains(&str_link) && !links_visited.contains(&str_link) {
    //             // add to queue        
    //             links_to_scrape.push(new_link.as_ref().to_string());
    //         }
    //     }

    //     for (key, value) in &link_content_hash {
    //         println!("Key: {}, Value: {}", key, value);
    //     }

    //     for link in &links_to_scrape {
    //         println!("{}", link);
    //     }
    // }


}
