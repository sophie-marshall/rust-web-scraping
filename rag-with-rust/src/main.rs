use polars::series;
use regex::Regex;
mod functions;
use functions::{get_links, get_content};
use polars::prelude::*;
// use reqwest::blocking::get;
use scraper::{Selector};
use csv::Writer;
use std::fs::File;
// use url::Url;


fn main() {

    // GET LIST OF URLS TO SCRAPE // 
    let homepage_url = "https://help.pbs.org/";

    let mut master_links: Vec<String> = match get_links(homepage_url, homepage_url) {
        Ok(links) => links,
        Err(err) => {
            println!("Error extracting links from homepage: {:?}", err);
            vec![]
        }
    };

    
    // iterate through list 
    for link in master_links.clone() {
        match get_links(&link, homepage_url) {
            Ok(links) => {
                // check if exists alread 
                for new_link in links {
                    if !master_links.contains(&new_link) {
                        master_links.push(new_link);
                    }
                }
            }
            Err(err) => {
                println!("Error extracting links from {}: {}:?", link, err);
            }
        }
    }
    
    // SCRAPE LINKS 
    let mut all_content = Vec::new(); 

    for link in &master_links{
        match get_content(&link) {
            Ok(content) => all_content.push(content),
            Err(err) => {
                println!("Error fetching content for {}: {:?}", link, err);
                all_content.push(" ".to_string());
                continue;
            }
        }
    }

    // Path to the output CSV file on the desktop
    let file_path = "C:/Users/Sophie/Desktop/data-dump/help_scrape_0110.csv";

     // Create a CSV writer
     match Writer::from_path(file_path) {
        Ok(mut writer) => {
            // Write headers (if needed)
            let _ = writer.write_record(&["Content", "Link"]);

            // Write data to the CSV file
            for (content, link) in all_content.iter().zip(master_links.iter()) {
                let _ = writer.write_record(&[content, link]);
            }

            if let Err(err) = writer.flush() {
                eprintln!("Failed to flush writer: {}", err);
            }
        }
        Err(err) => eprintln!("Error creating CSV writer: {}", err),
    }

}