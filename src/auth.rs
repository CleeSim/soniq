//! Authentication-related functionality for Last.fm API.

use std::collections::BTreeMap;

use serde::Deserialize;
use tracing::instrument;

use crate::client::Client;
use crate::error::Error;

/// Response from `auth.getToken`
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

/// Response from `auth.getSession`
#[derive(Debug, Deserialize)]
pub struct SessionResponse {
    pub session: Session,
}

/// A Last.fm session returned after authentication.
#[derive(Debug, Deserialize)]
pub struct Session {
    pub name: String,
    pub key: String,
    pub subscriber: u32,
}

impl Client {
    /// Get an authentication token.
    ///
    /// This token must be approved by the user before exchanging it for a session.
    ///
    /// [Last.fm docs](https://www.last.fm/api/show/auth.getToken)
    #[instrument(skip(self))]
    pub async fn get_token(&self) -> Result<String, Error> {
        let params = BTreeMap::new();
        let response: TokenResponse = self.signed_post("auth.getToken", params).await?;
        Ok(response.token)
    }

    /// Get a session key after the user has approved the token.
    ///
    /// This is used to perform authenticated requests on behalf of a user.
    ///
    /// [Last.fm docs](https://www.last.fm/api/show/auth.getSession)
    #[instrument(skip(self))]
    pub async fn get_session(&self, token: &str) -> Result<Session, Error> {
        let mut params = BTreeMap::new();
        params.insert("token".into(), token.into());

        let response: SessionResponse = self.signed_post("auth.getSession", params).await?;
        Ok(response.session)
    }
}
