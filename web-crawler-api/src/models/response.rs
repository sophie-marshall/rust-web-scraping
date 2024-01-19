use serde::{Deserialize, Serialize};

// define response struct to respond to client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub message: String,
}