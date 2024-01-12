
use scraper::{Html, Selector};
use regex::Regex;

pub fn get_content(html: &Html) -> String {

    // copy 
    let document = html.clone();

    // set selector and regex 
    let p_selector = Selector::parse("p").unwrap();
    let regex = Regex::new(r"[ \t\n]{2,}").unwrap();

    // instantiate string to hold content 
    let mut content = String::new();

    // append matches to content 
    for element in document.select(&p_selector) {
        let text = element.text().collect::<String>();
        println!("Text: {}", text);
        let clean_text = regex.replace_all(&text, " ");

        content.push_str(&clean_text)
    }

    // return content
    content

}