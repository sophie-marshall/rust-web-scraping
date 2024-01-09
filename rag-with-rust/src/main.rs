
// define webpage content data structure
struct WebpageContent {
    url: Option<String>,
    title: Option<String>,
    content: Option<String>,
    links: Option<String>,
}

fn main() {

    // retrieve html resposne, download content, and create a document
    let response = reqwest::blocking::get("https://docs.pbs.org/display/B3");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    
    // extract data from document
    let first_page = "https://docs.pbs.org/display/B3";
    
    // instantiate vector to hold pages
    let mut pages_to_scrape: Vec<String> = vec![first_page.to_owned()];
    let mut pages_discovered: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut i = 1;
    max_iter = 2;

    while !pages_to_scrape.is_empty() && i <= max_iter {
        //get first element from pages to scrape
        let page_to_scrape = pages_to_scrape.remove(0);

        // get doc and parse it 
        let response = reqwest::blocking::get(page_to_scrape);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        // apply scraping logic 
        let html_pagination_selector = scraper::Selector::parse("div#rw_item_content")
    }
}
