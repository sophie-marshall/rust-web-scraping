// import relevant structs 
use spider::tokio;
use spider::website::Website;
mod functions;
use functions::get_content;
use scraper::Html;
use std::collections::HashMap;
use polars::prelude::*;


#[tokio::main]
async fn main() {

    let mut queue: Vec<String> = Vec::new();
    let mut visited: Vec<String> = Vec::new();

    let mut website_hash: HashMap<String, String> = HashMap::new();

    // add base url to queue to start
    queue.push("https://help.pbs.org".to_string());

    // provided there are still links to be scraped
    while let Some(url) = queue.pop() {

        println!("Current Queue Length: {}", queue.len());

        // // check on hash
        // for (key, value) in &website_hash {
        //     println!("Key: {}, Value: {}", key, value);
        // };

        // add popped url to visited
        visited.push(url.clone());

        // crawl current url 
        let mut website: Website = Website::new(&url);
        website.scrape().await;

        // for each page found on the current website ... 
        for page in website.get_pages().unwrap().iter() {

            let link = page.get_url_final().to_string();

            // check if the page has been visited before 
            if !queue.contains(&link) && !visited.contains(&link) {

                // if it hasnt been visited and isnt in the queue...
                // add it to visited and get the content
                visited.push(page.get_url_final().to_string());
                let html_content = page.get_html();
                let document = Html::parse_document(&html_content);
                let page_content = get_content(&document);

                // insert information into hashmap
                website_hash.insert(link.clone(), page_content);

                // check for new links and add if not already in list
                let mut new_webstie: Website = Website::new(&link);
                new_webstie.crawl_smart().await;

                for link in new_webstie.get_links() {
                    let str_link = link.as_ref().to_string();
                    if !queue.contains(&str_link) && !visited.contains(&str_link) {
                        // println!("{}", &str_link);
                        queue.push(str_link);
                    }
                }
                

            }

        }

    }

    println!("Queue Empty!");

    // convert hashmap into table 
    let (keys, values): (Vec<String>, Vec<String>) = website_hash.into_iter().unzip();

    // Create Series from Vecs
    let keys_series = Series::new("key", keys);
    let values_series = Series::new("value", values);

    // Build the DataFrame
    let df = DataFrame::new(vec![keys_series, values_series]);

    // Display the Polars DataFrame
    println!("{:?}", df);

}