use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::models::url::Url;

// create sample DB to hold urls -- eventually this will hold crawled web data
pub struct Database {
    pub urls: Arc<Mutex<Vec<Url>>>,
}

// implement a new instance of database
impl Database {
    pub fn new() -> Self {
        let urls = Arc::new(Mutex::new(vec![]));
        Database { urls }
    }

    //
    pub fn create_url(&self, url: Url) -> Result<Url, Error> {
        // lock url so its only available to one thread at a time
        let mut urls = self.urls.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let url = Url {
            id: Some(id),
            ..url
        };
        // add new url to the urls vector
        urls.push(url.clone());
        // return new url struct to the caller 
        Ok(url)
    }
}

