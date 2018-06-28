extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod errors;
mod response;

use errors::LeanpubResult;
pub use response::Summary;

pub struct Client {
    slug: String,
    api_key: Option<String>,
}

impl Client {
    pub fn new<T: Into<String>>(slug: T, api_key: Option<T>) -> Self {
        return Client { 
            slug: slug.into(), 
            api_key: match api_key {
                Some(n) => Some(n.into()),
                None => None
            } 
        };
    }

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
