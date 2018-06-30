extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod responses;
mod utils;

pub use responses::Summary;
use utils::errors::LeanpubResult;
use utils::http::{DefaultHttpClient, HttpClient};

/// The Leanpub API client.
pub struct Client {
    client: Box<HttpClient>,
    api_key: Option<String>,
}

impl Client {
    /// Creates a new client for a specific slug.
    /// The API key is optional but required when
    /// interacting with API endpoints that requires
    /// an authenticated user.
    pub fn new(api_key: Option<String>) -> Self {
        return Client {
            client: Box::new(DefaultHttpClient {}),
            api_key,
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
        let response = self.client.get(&uri[..])?;
        return Ok(serde_json::from_str::<Summary>(&response)?);
    }

    fn append_api_key(&self, url: String) -> String {
        return match &self.api_key {
            Some(key) => format!("{}?api_key={}", url, key),
            None => url,
        };
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use utils::http::HttpClient;
    use utils::errors::LeanpubResult;

    pub struct FakeHttpClient { 
        content: String
    }
    impl HttpClient for FakeHttpClient {
        fn get(&self, _uri: &str) -> LeanpubResult<String> {
            return Ok(self.content.clone());
        }
    }

    #[test]
    fn lol() {
        // Given
        let client = Client {
            api_key: None,
            client: Box::new(FakeHttpClient {
                content: include_str!("res/authenticated.json").to_string(),
            })
        };
        
        // When
        let result = client.get_summary("lol").unwrap();

        // Then
        assert_eq!(321, result.id);
        assert_eq!("Qux", result.title);
        assert_eq!("A story about metasyntactic variable names", result.subtitle.as_ref().unwrap());
        assert_eq!("qux", result.slug);
        assert_eq!(None, result.about_the_book);
        assert_eq!(0.0, result.total_revenue);
        assert_eq!(None, result.last_published_at);
        assert_eq!(16645, result.word_count);
        assert_eq!(432, result.page_count);
        assert_eq!(0, result.word_count_published);
        assert_eq!(0, result.page_count_published);
        assert_eq!(0, result.total_copies_sold);
        assert_eq!(None, result.meta_description);
        assert_eq!("John Doe", result.author_string);
        assert_eq!("http://leanpub.com/qux", result.url);
        assert_eq!("https://s3.amazonaws.com/titlepages.leanpub.com/qux/original?123", result.title_page_url);
        assert_eq!(4.99, result.minimum_price);
        assert_eq!(9.99, result.suggested_price);
        assert_eq!("https://s3.amazonaws.com/titlepages.leanpub.com/qux/medium?123", result.image);
        assert_eq!(2, result.possible_reader_count);
        assert_eq!("http://leanpub.com/s/qux.pdf", result.pdf_preview_url.as_ref().unwrap());
        assert_eq!("http://leanpub.com/s/qux.epub", result.epub_preview_url.as_ref().unwrap());
        assert_eq!("http://leanpub.com/s/qux.mobi", result.mobi_preview_url.as_ref().unwrap());
    }
}
