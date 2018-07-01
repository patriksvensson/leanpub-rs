#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod responses;
#[macro_use]
mod utils;

use std::path::Path;

pub use responses::Summary;
use utils::errors::LeanpubResult;
use utils::http::{DefaultHttpClient, HttpClient};

/// The Leanpub API client.
pub struct Client {
    client: Box<HttpClient>,
    api_key: Option<String>,
}

/// Represents the different preview formats.
pub enum PreviewFormat {
    /// PDF format.
    Pdf,
    /// Epub format.
    Epub,
    /// Mobi format.
    Mobi,
}

impl Client {
    /// Creates a new client for a specific slug.
    /// The API key is optional but required when
    /// interacting with API endpoints that requires
    /// an authenticated user.
    pub fn new(api_key: Option<&str>) -> Self {
        return Client {
            client: Box::new(DefaultHttpClient {}),
            api_key: match api_key {
                Some(key) => Option::Some(key.to_string()),
                None => None,
            },
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

    /// Downloads the latest preview in the specified format.
    pub fn download_preview(
        &self,
        slug: &str,
        file_path: &Path,
        format: PreviewFormat,
    ) -> LeanpubResult<()> {
        let result = self.get_summary(slug)?;

        let url = match format {
            PreviewFormat::Pdf => result.pdf_preview_url,
            PreviewFormat::Epub => result.epub_preview_url,
            PreviewFormat::Mobi => result.mobi_preview_url,
        };

        match url {
            Some(url) => {
                // Download the file.
                self.client.download(&url.as_str(), file_path)?;
                return Ok(());
            }
            None => return Err(format_err!("No preview available.")),
        }
    }

    fn append_api_key(&self, url: String) -> String {
        return match &self.api_key {
            Some(key) => format!("{}?api_key={}", url, key),
            None => url,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_authenticated_summary_response_correctly() {
        // Given
        let client = create_fake_client!(
            Option::Some("api-key".to_string()),
            include_str!("res/authenticated.json"));

        // When
        let result = client.get_summary("qux").unwrap();

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
