use serde::{de, Deserialize, Deserializer};

/// Contains summary information about a book.
#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    /// The book's ID.
    pub id: u32,
    /// The book's title.
    pub title: String,
    /// The book's subtitle.
    #[serde(deserialize_with = "de_empty_str_to_optional_str")]
    pub subtitle: Option<String>,
    /// The book's slug.
    pub slug: String,
    /// The book's description.
    #[serde(deserialize_with = "de_empty_str_to_optional_str")]
    pub about_the_book: Option<String>,
    /// The book's total revenue.
    #[serde(default, deserialize_with = "de_str_to_f32")]
    pub total_revenue: f32,
    /// The date the book was last published.
    pub last_published_at: Option<String>,
    /// The number of words.
    #[serde(default)]
    pub word_count: u32,
    /// The number of pages.
    #[serde(default)]
    pub page_count: u32,
    /// The number of words in the published book.
    #[serde(default, deserialize_with = "de_nullable_int_to_u32")]
    pub word_count_published: u32,
    /// The number of pages in the published book.
    #[serde(default, deserialize_with = "de_nullable_int_to_u32")]
    pub page_count_published: u32,
    /// The number of total sold copies.
    #[serde(default)]
    pub total_copies_sold: u32,
    /// The meta description of the book.
    pub meta_description: Option<String>,
    /// Contains author information.
    pub author_string: String,
    /// The book's URL.
    pub url: String,
    /// A large image of the book's title page.
    pub title_page_url: String,
    #[serde(deserialize_with = "de_str_to_f32")]
    /// The book's minimum price.
    pub minimum_price: f32,
    #[serde(deserialize_with = "de_str_to_f32")]
    /// The book's suggested price.
    pub suggested_price: f32,
    /// A small image of the book's title page.
    pub image: String,
    #[serde(default)]
    /// The number of user's who has an interest in
    /// purchasing the book.
    pub possible_reader_count: u32,
    /// The URL to the PDF preview.
    pub pdf_preview_url: Option<String>,
    /// The URL to the epub preview.
    pub epub_preview_url: Option<String>,
    /// The URL to the mobi preview.
    pub mobi_preview_url: Option<String>,
}

fn de_str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    return s.parse::<f32>().map_err(de::Error::custom);
}

fn de_empty_str_to_optional_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    if s == "" {
        return Ok(Option::None);
    }
    return Ok(Option::Some(s));
}

fn de_nullable_int_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer)
        .map(|x: Option<_>| {
            x.unwrap_or(0)
        })
}