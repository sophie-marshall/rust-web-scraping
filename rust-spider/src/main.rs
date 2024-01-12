// import relevant structs 
use spider::tokio;
use spider::website::Website;
use scraper::{Html, html};
mod functions;
use functions::get_content;
use std::collections::HashMap;
use polars::prelude::*;
use polars::prelude::{
    CsvReader, CsvWriter, DataFrame,DataType, Field, JoinType, Schema,
    SerReader, SerWriter,
};
use std::fs::File;
// use std::path::Path;
// use std::sync::Arc;


#[tokio::main]
async fn main() {

    // define hashmap
    let mut hashmap: HashMap<String, String> = HashMap::new(); // SM: Revisit, is this the right struct?

    let url = "https://help.pbs.org/support/home";

    let mut website = Website::new(&url);

    // website
    //     .with_respect_robots_txt(true)
    //     .with_delay();

    website.scrape().await;

    // add links to scraping queue
    for page in website.get_pages().unwrap().iter() {

        // save link
        let link = page.get_url_final().to_string();

        // get paragraph content
        let html_content = page.get_html();
        let page_content = get_content(&Html::parse_document(&html_content));

        // add to hash map 
        // hashmap.insert(link, page_content);

    }

    // convert hashmap into table 
    let (keys, values): (Vec<String>, Vec<String>) = hashmap.into_iter().unzip();

    // Create Series from Vecs
    let keys_series = Series::new("key", keys);
    let values_series = Series::new("value", values);

    // Build the DataFrame
    let df = DataFrame::new(vec![keys_series, values_series]);

    // Display the Polars DataFrame
    println!("{:?}", df);

    let filepath = "/Users/srmarshall/Desktop/test.csv";

    let mut file = File::create(filepath).expect("could not create file");
    CsvWriter::new(&mut file)
        .has_headers(true)
        .with_delimiter(b',')
        .finish(&mut df.unwrap())
        .expect("failed to write output");


    
}