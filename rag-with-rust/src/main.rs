mod functions;
use functions::{get_webpage_content};
use polars::prelude::*;
use regex::Regex;


fn main() {

    match get_webpage_content("https://docs.pbs.org/display/B3") {

        Ok(content) => {

            let space_regex = Regex::new(r"[ \t\n]{2,}").unwrap();
            let cleaned_content = content.content.map(|s| space_regex.replace_all(&s, " ").to_string());

            // Create a Polars DataFrame
            let df = DataFrame::new(vec![
                Series::new("URL", &[content.url.unwrap_or_default()]),
                Series::new("Title", &[content.title.map(|s| s.replace(char::is_whitespace, "")).unwrap_or_default()]),
                Series::new("Content", &[cleaned_content.unwrap_or_default()]),
                Series::new("Links", &[content.links.join(", ")])

            ])

            .expect("Failed to create DataFrame");

            // Print the Polars DataFrame
            println!("{:?}", df);

        },
        Err(err) => eprintln!("Error: {:?}", err),
    }



}