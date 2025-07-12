//! Soniq
//!
//! A Rust library to interact with the Last.fm API.

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod sig;
pub mod utils;

pub use crate::client::Client;
pub use crate::error::Error;
