extern crate curl;

use std::str;
use self::curl::easy::Easy;
use utils::errors::LeanpubResult;

pub trait HttpClient {
    fn get(&self, uri: &str) -> LeanpubResult<String>;
}

pub struct DefaultHttpClient { }
impl HttpClient for DefaultHttpClient {
    fn get(&self, uri: &str) -> LeanpubResult<String> {
        let mut handle = Easy::new();
        handle.follow_location(true)?; // Follow redirects.
        handle.url(uri)?; // Set the URL.

        let mut content: String = String::new();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                content.push_str(str::from_utf8(data).unwrap());
                Ok(data.len())
            })?;

            transfer.perform()?;
        }

        return Ok(content);
    }
}