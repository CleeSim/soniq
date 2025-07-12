//! Models for user-related Last.fm API responses.

use serde::Deserialize;

use crate::models::common::{Image, PaginationMeta};
use crate::utils::{from_str, from_str_opt};

/// Response wrapper from the API: `{ "user": { ... } }`
#[derive(Debug, Deserialize)]
pub struct UserGetInfoResponse {
    pub user: UserInfo,
}

/// Main user info object returned by `user.getInfo`
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub realname: Option<String>,
    pub url: String,
    pub country: Option<String>,

    #[serde(deserialize_with = "from_str_opt")]
    pub age: Option<u32>,

    pub gender: Option<String>,

    #[serde(deserialize_with = "from_str")]
    pub playcount: u64,

    #[serde(deserialize_with = "from_str_opt")]
    pub playlists: Option<u32>,

    pub registered: Registered,
    pub image: Vec<Image>,

    #[serde(rename = "type")]
    pub user_type: Option<String>,

    #[serde(deserialize_with = "from_str_opt")]
    pub subscriber: Option<u8>,
}

/// Info about when the user registered their account.
#[derive(Debug, Deserialize)]
pub struct Registered {
    #[serde(rename = "#text")]
    pub timestamp: i64,

    #[serde(deserialize_with = "from_str")]
    pub unixtime: i64,
}

/// Response wrapper from the API: `{ "friends": { ... } }`
#[derive(Debug, Deserialize)]
pub struct UserGetFriendsResponse {
    pub friends: UserFriends,
}

/// A list of friends and pagination info.
#[derive(Debug, Deserialize)]
pub struct UserFriends {
    #[serde(rename = "@attr")]
    pub attr: PaginationMeta,
    pub user: Vec<Friend>,
}

/// A single friend object.
#[derive(Debug, Deserialize)]
pub struct Friend {
    pub name: String,
    pub url: String,
    pub country: Option<String>,
    pub realname: Option<String>,

    #[serde(deserialize_with = "from_str")]
    pub playlists: u32,

    #[serde(deserialize_with = "from_str")]
    pub playcount: u64,

    pub image: Vec<Image>,
    pub registered: FriendRegistered,

    #[serde(deserialize_with = "from_str")]
    pub subscriber: u8,

    #[serde(rename = "type")]
    pub user_type: String,

    #[serde(deserialize_with = "from_str")]
    pub bootstrap: u8,
}

/// Info about when a friend registered their account.
///
/// This is different from the [`Registered`] struct used in [`UserInfo`]
/// for some reason.
#[derive(Debug, Deserialize)]
pub struct FriendRegistered {
    #[serde(rename = "#text")]
    pub date: String,

    #[serde(deserialize_with = "from_str")]
    pub unixtime: i64,
}

/// Response wrapper for loved tracks: `{ "lovedtracks": { ... } }`
#[derive(Debug, Deserialize)]
pub struct UserGetLovedTracksResponse {
    pub lovedtracks: LovedTracks,
}

/// A list of loved tracks and pagination info.
#[derive(Debug, Deserialize)]
pub struct LovedTracks {
    #[serde(rename = "@attr")]
    pub attr: PaginationMeta,
    pub track: Vec<LovedTrack>,
}

/// A single loved track object.
#[derive(Debug, Deserialize)]
pub struct LovedTrack {
    pub artist: LovedTrackArtist,
    pub name: String,
    pub url: String,
    pub date: LovedTrackDate,
    pub image: Vec<Image>,
    pub streamable: Streamable,
    pub mbid: Option<String>,
}

/// Artist info for a loved track.
#[derive(Debug, Deserialize)]
pub struct LovedTrackArtist {
    pub name: String,
    pub mbid: Option<String>,
    pub url: String,
}

/// Date info for when a track was loved.
#[derive(Debug, Deserialize)]
pub struct LovedTrackDate {
    pub uts: String,
    #[serde(rename = "#text")]
    pub text: String,
}

/// Streamability info for a loved track.
#[derive(Debug, Deserialize)]
pub struct Streamable {
    #[serde(deserialize_with = "from_str")]
    pub fulltrack: u8,
    #[serde(rename = "#text", deserialize_with = "from_str")]
    pub is_streamable: u8,
}
