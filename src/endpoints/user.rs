//! User API methods for Last.fm.

use std::collections::BTreeMap;

use crate::{
    client::Client,
    error::Error,
    models::user::{UserFriends, UserGetFriendsResponse, UserGetInfoResponse, UserInfo},
};

/// Extension trait that provides user-related API methods.
pub trait UserEndpointExt {
    fn user(&self) -> UserHandler<'_>;
}

/// Implements `user()` on the client.
impl UserEndpointExt for Client {
    fn user(&self) -> UserHandler<'_> {
        UserHandler { client: self }
    }
}

/// Handles `user.*` Last.fm API methods.
#[derive(Debug)]
pub struct UserHandler<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> UserHandler<'a> {
    /// Get info for a Last.fm user.
    ///
    /// [API Reference](https://www.last.fm/api/show/user.getInfo)
    pub async fn get_info(&self, username: &str) -> Result<UserInfo, Error> {
        let mut params = BTreeMap::new();
        params.insert("user".into(), username.to_string());

        let response: UserGetInfoResponse =
            self.client.unsigned_get("user.getInfo", params).await?;

        Ok(response.user)
    }

    /// Get a list of friends for a Last.fm user.
    ///
    /// [API Reference](https://www.last.fm/api/show/user.getFriends)
    pub async fn get_friends(&self, username: &str) -> Result<UserFriends, Error> {
        let mut params = BTreeMap::new();
        params.insert("user".into(), username.to_string());

        let response: UserGetFriendsResponse =
            self.client.unsigned_get("user.getFriends", params).await?;

        Ok(response.friends)
    }
}
