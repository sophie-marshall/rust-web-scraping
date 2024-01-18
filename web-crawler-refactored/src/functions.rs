use spider::page::Page;
use scraper::{Html, Selector};
use regex::Regex;
use std::error::Error;
use csv::Writer;
use url::Url;
use chrono::Local;
use rusoto_core::{Region, HttpClient};
use rusoto_s3::{S3, S3Client, PutObjectRequest};
use dotenv::dotenv;
use std::env;

// must start with crate self or super bc its a visibility modifier
use crate::models::WebpageData;

fn parse_html(html_content: &str) -> String {
    // logic for parsing HTML content

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
    let filename = format!("{}__{}__crawl_data.csv", path_segment, date);
    // set full filepath
    let filepath = format!("/Users/Sophie/Desktop/{}", filename);

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

// function to put object in s3
pub async fn write_to_s3(data: Vec<WebpageData>) -> Result<(), Box<dyn std::error::Error>> {
    // load .env
    dotenv().ok();

    // set aws credentials
    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID");
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY");
    let region = Region::UsEast1;

    // initialize s3 client
    let s3 = S3Client::new_with(
        HttpClient::new().unwrap(),
        aws_hyper::Standars,
        region.clone(),
    );

    // convert data to csv
    let csv = data
        .iter()
        .map(|data| format!("{},{},{}\n", data.link, data.html_content, data.parsed_content))
        .collect::<String>();

    // set bucket
    let bucket = "scraper-webpage-data";

    // set bucket key
    let url = Url::parse(base_url)?;
    let url_id = url.host_str().unwrap_or_default();
    let date = Local::now().format("%m%d%y").to_string();  
    let key = format!("raw-data/{}__{}.csv", path_segment, date);

    // create request
    let request = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: key.to_owned(),
        body: Some(csv.into_bytes().into()),
        ..Default::default()
    };

    // upload to s3
    match s3.put_object(request).await {
        OK(_) => {
            println!("CSV successfully uploaded to S3");
            Ok(())
        }
        Err(err) => {
            eprintln!("Error uploading to S3: {}", errr);
            Err(err.into())
        }
    }

}
