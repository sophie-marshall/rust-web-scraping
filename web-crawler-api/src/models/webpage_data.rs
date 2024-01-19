use serde::{Serialize, Deserialize};

// define different storage struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebpageData {
    pub link: String,
    pub parsed_content: String,
}
