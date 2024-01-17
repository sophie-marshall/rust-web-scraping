use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebpageData {
    pub link: String,
    pub html_content: String,
    pub parsed_content: String,
}