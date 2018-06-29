extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod utils;
mod responses;

use utils::errors::{LeanpubResult};
pub use responses::Summary;

/// The Leanpub API client.
pub struct Client {
    api_key: Option<String>,
}

impl Client {
    /// Creates a new client for a specific slug.
    /// The API key is optional but required when 
    /// interacting with API endpoints that requires
    /// an authenticated user.
    pub fn new(api_key: Option<String>) -> Self {
        return Client {  
            api_key
        };
    }

    /// Gets the books summary information.
    /// 
    /// # Note
    /// 
    /// This does not require authentication. If you do provide an API key, 
    /// and you are an author of the book, then we will also give you the 
    /// total copies sold and revenue for the book, as well as the URLs for 
    /// downloading the current preview and published version of your book 
    /// in epub, pdf and mobi formats.
    pub fn get_summary(&self, slug: &str) -> LeanpubResult<Summary> {
        let uri = self.append_api_key(format!("https://leanpub.com/{}.json", slug));
        let response = utils::http::get(&uri[..])?;
        return Ok(serde_json::from_str::<Summary>(&response)?);
    }

    fn append_api_key(&self, url: String) -> String {
        return match &self.api_key {
            Some(key) => format!("{}?api_key={}", url, key),
            None => url,
        }
    }
}
