use spider::page::Page;
use scraper::{Html, Selector};
use regex::Regex;
use std::error::Error;
use csv::Writer;
use url::Url;
use chrono::Local;

// must start with crate self or super bc its a visibility modifier
use crate::models::WebpageData;

fn parse_html(html_content: &str) -> String {
    // transform into document
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("p").unwrap();
    let regex = Regex::new(r"[ \t\n]{2,}").unwrap();

    let mut content = String::new();

    for element in document.select(&selector) {
        let raw_text = element.text().collect::<String>();
        let clean_text = regex.replace_all(&raw_text, " ");
        content.push_str(&clean_text);
    }

    // return content as a string
    content

}

// define function to process data from pages
pub fn extract_webpage_data(page: &Page) -> WebpageData {

    // get link and html_content directly from page
    let link = page.get_url_final().to_string();
    let html_content = page.get_html();
    let parsed_content = parse_html(&html_content);

    // return a struct of processed data 
    WebpageData {
        link: link,
        html_content: html_content.to_string(), 
        parsed_content: parsed_content,
    }
}

// function to export to a local csv
pub fn write_csv(data: &[WebpageData], base_url: &str) -> Result<(), Box<dyn Error>> {

    // conver url 
    let url = Url::parse(base_url)?;

    // extract base url segmeent and set date for filename 
    let path_segment = url.host_str().unwrap_or_default();
    let date = Local::now().format("%m%d%y").to_string();  
    // set filename using vars above
    let filename = format!("{}__{}.csv", path_segment, date);
    // set full filepath
    let filepath = format!("/Users/srmarshall/Desktop/{}", filename);

    let mut csv_writer = Writer::from_path(filepath)?;

    csv_writer.write_record(vec!["link", "html_content", "parsed_content"])?;

    for entry in data {
        csv_writer.write_record(vec![
            entry.link.to_string(),
            entry.html_content.to_string(),
            entry.parsed_content.to_string(),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
