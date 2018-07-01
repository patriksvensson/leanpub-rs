extern crate curl;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::str;

use self::curl::easy::Easy;
use utils::errors::LeanpubResult;

pub trait HttpClient {
    fn get(&self, uri: &str) -> LeanpubResult<String>;
    fn download(&self, uri: &str, path: &Path) -> LeanpubResult<()>;
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

    fn download(&self, uri: &str, path: &Path) -> LeanpubResult<()> {
        let mut handle = Easy::new();
        handle.follow_location(true)?; // Follow redirects.
        handle.url(uri)?; // Set the URL.

        // Download the file.
        let mut file = File::create(path)?;
        handle.write_function(move |data| {
            return Ok(file.write(data).unwrap());
        })?;
        handle.perform()?;

        // Check the response code.
        let response = handle.response_code()?;
        if response != 200 {
            fs::remove_file(path)?; // Delete the file.
            return Err(format_err!(
                "Expected status code 200 but received {}.",
                response
            ));
        }

        return Ok(());
    }
}

// ------------------------------------------------------------------------------
// Test utilities
// ------------------------------------------------------------------------------

#[cfg(test)]
pub struct FakeHttpClient {
    content: String,
}

#[cfg(test)]
#[macro_export]
macro_rules! create_fake_client {
    ($api_key:expr, $content:expr) => {
        Client {
            api_key: $api_key,
            client: Box::new(utils::http::FakeHttpClient::new($content)),
        };
    };
}

#[cfg(test)]
impl FakeHttpClient {
    #[cfg(test)]
    pub fn new(content: &str) -> FakeHttpClient {
        return FakeHttpClient {
            content: content.to_string(),
        };
    }
}

#[cfg(test)]
impl HttpClient for FakeHttpClient {
    fn get(&self, _uri: &str) -> LeanpubResult<String> {
        return Ok(self.content.clone());
    }
    fn download(&self, _uri: &str, _path: &Path) -> LeanpubResult<()> {
        return Ok(());
    }
}