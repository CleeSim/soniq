//! Contains common structs and traits used across the Last.fm API models.

use serde::Deserialize;

/// Image with Last.fm `size` key
#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "#text")]
    pub url: String,
    pub size: Option<String>, // Can be: small, medium, large, extralarge.
}

/// For OpenSearch-like paging (used in search results)
#[derive(Debug, Deserialize)]
pub struct OpenSearchMeta {
    #[serde(rename = "opensearch:totalResults")]
    pub total_results: u64,
    #[serde(rename = "opensearch:startIndex")]
    pub start_index: u64,
    #[serde(rename = "opensearch:itemsPerPage")]
    pub items_per_page: u64,
}
