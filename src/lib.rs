extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod utils;
mod responses;

use utils::LeanpubResult;
pub use responses::Summary;

/// The Leanpub API client.
pub struct Client {
    slug: String,
    api_key: Option<String>,
}

impl Client {
    /// Creates a new client for a specific slug.
    /// The API key is optional but required when 
    /// interacting with API endpoints that requires
    /// an authenticated user.
    pub fn new<T: Into<String>>(slug: T, api_key: Option<T>) -> Self {
        return Client { 
            slug: slug.into(), 
            api_key: match api_key {
                Some(n) => Some(n.into()),
                None => None
            } 
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
    pub fn get_summary(&self) -> LeanpubResult<Summary> {
        let uri = self.append_api_key(format!("https://leanpub.com/{}.json", self.slug));
        let response = reqwest::get(&uri[..])?.text()?;
        return Ok(serde_json::from_str::<Summary>(&response)?);
    }

    fn append_api_key(&self, url: String) -> String {
        return match &self.api_key {
            Some(key) => format!("{}?api_key={}", url, key),
            None => url,
        }
    }
}
