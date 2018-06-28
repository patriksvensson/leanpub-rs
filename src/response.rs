use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub id: u32,
    pub title: String,
    #[serde(deserialize_with = "de_empty_str_to_optional_str")]
    pub subtitle: Option<String>,
    pub slug: String,
    #[serde(deserialize_with = "de_empty_str_to_optional_str")]
    pub about_the_book: Option<String>,
    #[serde(default, deserialize_with = "de_str_to_f32")]
    pub total_revenue: f32,
    pub last_published_at: Option<String>,
    #[serde(default)]
    pub word_count: u32,
    #[serde(default)]
    pub page_count: u32,
    #[serde(default, deserialize_with = "de_nullable_int_to_u32")]
    pub word_count_published: u32,
    #[serde(default, deserialize_with = "de_nullable_int_to_u32")]
    pub page_count_published: u32,
    #[serde(default)]
    pub total_copies_sold: u32,
    pub meta_description: Option<String>,
    pub author_string: String,
    pub url: String,
    pub title_page_url: String,
    #[serde(deserialize_with = "de_str_to_f32")]
    pub minimum_price: f32,
    #[serde(deserialize_with = "de_str_to_f32")]
    pub suggested_price: f32,
    pub image: String,
    #[serde(default)]
    pub possible_reader_count: u32,
    pub pdf_preview_url: Option<String>,
    pub epub_preview_url: Option<String>,
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