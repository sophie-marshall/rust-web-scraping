use serde::{Serialize, Deserialize};
use crate::models::webpage_data::WebpageData;

// define response object 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrawledDataResponse {
    pub crawled_data: Vec<WebpageData>,
}
