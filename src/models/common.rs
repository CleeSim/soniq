//! Contains common structs and traits used across the Last.fm API models.

use serde::Deserialize;

use crate::utils::from_str;

/// Image with Last.fm `size` key
#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "#text")]
    pub url: String,
    pub size: Option<String>, // Can be: small, medium, large, extralarge.
}

/// Meta information for paginated responses.
#[derive(Debug, Deserialize)]
pub struct PaginationMeta {
    pub user: String,

    #[serde(rename = "totalPages", deserialize_with = "from_str")]
    pub total_pages: u32,

    #[serde(deserialize_with = "from_str")]
    pub page: u32,

    #[serde(rename = "perPage", deserialize_with = "from_str")]
    pub per_page: u32,

    #[serde(deserialize_with = "from_str")]
    pub total: u32,
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
