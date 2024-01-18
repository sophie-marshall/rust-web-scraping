use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    pub id: Option<String>,
    pub url: String,
}