use serde::{Deserialize, Serialize};

// define response struct to respond to client
#[derive(Serialize, Deserialize)]
pub struct Input {
    pub message: String,
}