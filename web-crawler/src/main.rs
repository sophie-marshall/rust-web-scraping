mod functions;
use functions::{get_content, get_links};
use spider::case_insensitive_string::serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use serde::Serialize;
use csv::Writer;

// define struct for website data 
#[derive(Debug, Serialize, Deserialize)]
struct CrawledData {
    link: String,
    content: String,
}

fn main() {
    // define storage
    let mut crawled_data: Vec<CrawledData> = Vec::new();
    // define base url to crawl 
    let base_url = "https://help.pbs.org/";
    // crawl base url for associated links
    let mut link_queue = get_links(&base_url).unwrap();
    // break after first crawl
    sleep(Duration::from_secs(30));
    // instantiate counter for breaking logic latter
    let mut counter = 0;
    // process links until queue is empty
    while let Some(link) = link_queue.pop() {
        // throttle requests to comply with site limits
        if counter % 5 == 0 && counter > 0 {
            println!("------------ BREAK: {} Links Remaining ------------", link_queue.len());
            sleep(Duration::from_secs(10));
        }
        // fetch content 
        let content = get_content(&link);
        // if still hitting rate limit append link back to queue
        if content.is_empty() {
            println!("Implementing site specific break: {}", &link);
            sleep(Duration::from_secs(30));
        } else {
            // create CrawledData instance + add to vec
            let data = CrawledData{ link, content };
            crawled_data.push(data);
        }
        // increment for tracking
        counter += 1
    }
    // define filepath
    let filepath = "/Users/srmarshall/Desktop/text_4_retry_logic.csv";
    // create file
    let file = File::create(filepath).expect("could not create file");
    // write to csv 
    let mut csv_writer = Writer::from_writer(file);
    for entry in crawled_data {
        csv_writer.serialize(entry).expect("Failed to write CSV")
    }
}
