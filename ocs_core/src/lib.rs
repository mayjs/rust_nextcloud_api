pub mod core;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use thiserror::Error;
use reqwest::{Client, header, IntoUrl, Url};

/// Common error type for the Nextcloud API
#[derive(Error, Debug)]
pub enum NextcloudApiError {
    #[error("response format invalid")]
    SerdeError(#[from] serde_json::Error),
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("capability not found")]
    MissingCapabilityError,
}

/// Meta information that is part of every OCS API response
#[derive(Deserialize, Debug)]
pub struct ApiResponseMeta {
    pub status: String,
    pub statuscode: u32,
    pub message: String,
    pub totalitems: String,
    pub itemsperpage: String,
}

/// Overall OCS API response structure
#[derive(Deserialize, Debug)]
pub struct ApiResponse<D> {
    pub meta: ApiResponseMeta,
    pub data: D
}

/// Wrapper for the ocs root element in OCS API responses
#[derive(Deserialize)]
struct OcsWrapper<T> {
    ocs: T,
}

/// Result type with a fixed error type
pub type Result<T> = std::result::Result<T, NextcloudApiError>;
/// A result that contains an OCS API response
pub type ApiResult<T> = Result<ApiResponse<T>>;

/// Credentials for the nextcloud API
pub struct NextcloudCredentials {
    /// Username/ID
    user: String,
    /// Password
    password: String,
}

impl NextcloudCredentials {
    pub fn new(user: String, password: String) -> Self {
        Self{ user, password }
    }
}

/// The main Nextcloud API client.
///
/// Most user-faced functionality is implemented through extension traits.
pub struct NextcloudApiClient {
    url: Url,
    credentials: NextcloudCredentials,
    api_client: Client,
}

impl NextcloudApiClient {
    /// Construct a new API client for the given URL and credentials
    pub fn new<U>(url: U, credentials: NextcloudCredentials) -> Result<Self>
    where U: IntoUrl {
        let mut headers = header::HeaderMap::new();
        headers.insert("OCS-ApiRequest", header::HeaderValue::from_static("true"));
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));
        let api_client = Client::builder().default_headers(headers).build()?;
        Ok(Self { url: url.into_url()?, credentials, api_client })
    }

    /// Prepare an authenticated request to the API using the given method and path
    fn authenticated_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let full_url = self.url.join(path).expect("Invalid API path");
        self.api_client.request(method, full_url).basic_auth(&self.credentials.user, Some(&self.credentials.password))
    }

    /// Send a GET request to the given path, deserializing the response into a given type T
    ///
    /// This function has to be used on OCS endpoints, where the entire result is wrapped in a
    /// `ocs` root.
    pub async fn ocs_get<T>(&self, path: &str) -> Result<T>
    where T: DeserializeOwned {
        Ok(self.api_get::<OcsWrapper<_>>(path)
            .await?
            .ocs)
    }

    /// Send a get request to some path, the return value is convered to `T` without any steps in
    /// between.
    pub async fn api_get<T>(&self, path: &str) -> Result<T>
    where T: DeserializeOwned {
        Ok(self.authenticated_request(reqwest::Method::GET, path)
            .send()
            .await?
            .json()
            .await?
            )
    }

}
