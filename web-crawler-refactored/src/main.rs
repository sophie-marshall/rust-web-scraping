use spider::tokio;

mod models;
pub use models::WebpageData;

mod crawler_config;
use crawler_config::configure_crawler;

mod functions;
use functions::{extract_webpage_data, write_csv, upload_to_s3};

#[tokio::main]
async fn main() {

    // define URL constant
    let base_url = "https://help.pbs.org/";

    let mut website = configure_crawler(base_url);

    website.scrape().await; // SM: add some type of error handling here

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

    // set vars for s3
    let bucket_name = "scraper-webpage-data";

    // // export to csv 
    // if let Err(err) = write_csv(&crawled_data, base_url) {
    //     eprintln!("Error exporting CSV: {}", err);
    // }


}
