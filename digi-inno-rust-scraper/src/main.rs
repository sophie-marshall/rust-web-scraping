mod functions;
use functions::{get_content, get_links};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;

fn main() {

    // define storage
    let mut hashmap: HashMap<String, String> = HashMap::new(); // SM: Revisit, is this the right struct?
    
    // define base url to crawl 
    let base_url = "https://help.pbs.org/";

    // crawl base url for associated links
    let mut link_queue = get_links(&base_url).unwrap();

    sleep(Duration::from_secs(30));

    let mut counter = 0;

    // as long as queue is filled
    while let Some(link) = link_queue.pop() {

        // implement break if needed 
        if counter % 10 == 0 && counter > 0 {
            println!("------------ BREAK ({} Links Remaining) ------------", link_queue.len());
            sleep(Duration::from_secs(10));
        }

        // fetch content 
        let content = get_content(&link);

        // push to hash
        hashmap.insert(link.clone(), content);

        counter += 1
    }

    for value in hashmap.values() {
        println!("Value: {}", value);
    }
}
