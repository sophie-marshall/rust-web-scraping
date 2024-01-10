use reqwest;
use scraper::{Html, Selector};
use url::Url;
use regex::Regex;
// use reqwest::blocking::get;


// function to get links from a webpage 
pub fn get_links(url: &str, base_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    // get webpage content 
    let response = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&response);

    // get base url 
    let base_url = Url::parse(base_url)?;

    // get links 
    let links: Vec<String> = document
        .select(&scraper::Selector::parse("a").unwrap())
        .filter_map(|a| {
            if let Some(href) = a.value().attr("href") {
                if let Ok(url) = base_url.join(href) {
                    Some(url.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // return links 
    Ok(links)

}

// function to get webpage content 
pub fn get_content(url: &str) -> Result<String, reqwest::Error> {

    // get webpage content 
    let response = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&response);

    // set paragraph selector 
    let paragraph_selector = Selector::parse("p").unwrap();

    // define regex for celanup 
    let xtra_space_regex = Regex::new(r"[ \t\n]{2,}").unwrap();

    // instantiate string to hold paragraph content
    let mut content = String::new();

    for p_element in document.select(&paragraph_selector) {
        let text = p_element.text().collect::<String>();
        let clean_text = xtra_space_regex.replace_all(&text, " ");
        content.push_str(&clean_text);
        content.push_str(". "); // Add a separator between paragraphs
    }

    Ok(content)
    
}