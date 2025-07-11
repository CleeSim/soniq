//! Client for interacting with the Last.fm API.

use std::collections::BTreeMap;
use std::time::Duration;

use reqwest::{Client as HttpClient, Url};
use serde::de::DeserializeOwned;
use tracing::instrument;

use crate::error::{Error, ErrorResponse};
use crate::sig::create_sig;

/// Default Last.fm API base URL.
const LASTFM_API_BASE: &str = "https://ws.audioscrobbler.com/2.0/";

/// Default User-Agent string.
const DEFAULT_USER_AGENT: &str = concat!(
    "soniq/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/CleeSim/soniq)"
);

/// Client to interact with the Last.fm API.
///
/// Use [`Client::builder()`] to construct a new client.
#[derive(Clone, Debug)]
pub struct Client {
    api_key: String,
    api_secret: Option<String>,
    http: HttpClient,
    base_url: Url,
}

impl Client {
    /// Creates a new builder with the mandatory API key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use soniq::client::Client;
    ///
    /// # async fn run() -> Result<(), soniq::Error> {
    /// let client = Client::builder("YOUR_API_KEY")
    ///     .api_secret("YOUR_API_SECRET")
    ///     .timeout(Duration::from_secs(5))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    /// Performs an unsigned GET request to the Last.fm API.
    #[instrument(skip(self, params))]
    pub async fn unsigned_get<T: DeserializeOwned>(
        &self,
        method: &str,
        mut params: BTreeMap<String, String>,
    ) -> Result<T, Error> {
        params.insert("method".into(), method.into());
        params.insert("api_key".into(), self.api_key.clone());
        params.insert("format".into(), "json".into());

        self.get(params).await
    }

    /// Performs a signed POST request without a user session key.
    ///
    /// This is for methods that require a signature but don't operate on a specific user's account.
    #[instrument(skip(self, params))]
    pub async fn signed_post<T: DeserializeOwned>(
        &self,
        method: &str,
        params: BTreeMap<String, String>,
    ) -> Result<T, Error> {
        self.signed_post_with_session(method, None, params).await
    }

    /// Performs a signed POST request with an optional user session key.
    ///
    /// Pass `Some(session_key)` to authenticate on behalf of a user.
    /// This requires the client to have been built with an API secret.
    #[instrument(skip(self, session_key, params))]
    pub async fn signed_post_with_session<T: DeserializeOwned>(
        &self,
        method: &str,
        session_key: Option<&str>,
        mut params: BTreeMap<String, String>,
    ) -> Result<T, Error> {
        let api_secret = self.api_secret.as_deref().ok_or(Error::MissingApiSecret)?;

        params.insert("method".into(), method.into());
        params.insert("api_key".into(), self.api_key.clone());

        if let Some(sk) = session_key {
            params.insert("sk".into(), sk.into());
        }

        let sig = create_sig(&params, api_secret);
        params.insert("api_sig".into(), sig);
        params.insert("format".into(), "json".into());

        self.post(params).await
    }

    /// Internal GET handler.
    async fn get<T: DeserializeOwned>(&self, params: BTreeMap<String, String>) -> Result<T, Error> {
        let res = self
            .http
            .get(self.base_url.clone())
            .query(&params)
            .send()
            .await?;
        Self::handle_response(res).await
    }

    /// Internal POST handler.
    async fn post<T: DeserializeOwned>(
        &self,
        params: BTreeMap<String, String>,
    ) -> Result<T, Error> {
        let res = self
            .http
            .post(self.base_url.clone())
            .form(&params)
            .send()
            .await?;
        Self::handle_response(res).await
    }

    /// Handles response and deserializes or returns errors accordingly.
    async fn handle_response<T: DeserializeOwned>(res: reqwest::Response) -> Result<T, Error> {
        let status = res.status();
        let text = res.text().await?;

        if status.is_success() {
            serde_json::from_str(&text).map_err(Error::from)
        } else {
            // Attempt to parse Last.fm's specific error format first.
            match serde_json::from_str::<ErrorResponse>(&text) {
                Ok(err) => Err(Error::LastFm(err)),
                // If that fails, it's likely an issue with the response format itself.
                Err(_) => Err(Error::Http { status, text }),
            }
        }
    }
}

impl Client {
    pub fn user(&self) -> crate::endpoints::user::UserHandler<'_> {
        crate::endpoints::user::UserEndpointExt::user(self)
    }
}

/// Builder for [`Client`].
#[derive(Debug)]
pub struct ClientBuilder {
    api_key: String,
    api_secret: Option<String>,
    timeout: Duration,
    user_agent: String,
    base_url: Url,
}

impl ClientBuilder {
    /// Creates a new builder with the mandatory API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: None,
            timeout: Duration::from_secs(10),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            base_url: Url::parse(LASTFM_API_BASE).expect("Default base URL is invalid?"),
        }
    }

    /// Sets the API secret, required for any signed API calls.
    pub fn api_secret(mut self, secret: impl Into<String>) -> Self {
        self.api_secret = Some(secret.into());
        self
    }

    /// Sets the request timeout for all API calls.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Sets the `User-Agent` header for all requests.
    // pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
    //     self.user_agent = agent.into();
    //     self
    // }

    /// Overrides the Last.fm API base URL (e.g., for testing or proxies).
    pub fn base_url(mut self, url: impl AsRef<str>) -> Result<Self, url::ParseError> {
        self.base_url = Url::parse(url.as_ref())?;
        Ok(self)
    }

    /// Builds the `Client`.
    pub fn build(self) -> Result<Client, Error> {
        let http = HttpClient::builder()
            .timeout(self.timeout)
            .user_agent(self.user_agent)
            .build()?;

        Ok(Client {
            api_key: self.api_key,
            api_secret: self.api_secret,
            http,
            base_url: self.base_url,
        })
    }
}
