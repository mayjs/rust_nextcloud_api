use serde::de::DeserializeOwned;
use serde::Deserialize;
use thiserror::Error;
use reqwest::{Client, header, IntoUrl, Url};
use serde_json::Value;

#[derive(Error, Debug)]
pub enum NextcloudApiError {
    #[error("response format invalid")]
    SerdeError(#[from] serde_json::Error),
    #[error("network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("capability not found")]
    MissingCapabilityError,
}

pub type Result<T> = std::result::Result<T, NextcloudApiError>;
pub type ApiResult<T> = Result<ApiResponse<T>>;

pub struct NextcloudCredentials {
    user: String,
    password: String,
}

impl NextcloudCredentials {
    pub fn new(user: String, password: String) -> Self {
        Self{ user, password }
    }
}

pub struct NextcloudApiClient {
    url: Url,
    credentials: NextcloudCredentials,
    api_client: Client,
}

#[derive(Deserialize)]
struct OcsWrapper<T> {
    ocs: T,
}

impl NextcloudApiClient {
    pub fn new<U>(url: U, credentials: NextcloudCredentials) -> Result<Self>
    where U: IntoUrl {
        let mut headers = header::HeaderMap::new();
        headers.insert("OCS-ApiRequest", header::HeaderValue::from_static("true"));
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));
        let api_client = Client::builder().default_headers(headers).build()?;
        Ok(Self { url: url.into_url()?, credentials, api_client })
    }

    fn authenticated_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let full_url = self.url.join(path).expect("Invalid API path");
        self.api_client.request(method, full_url).basic_auth(&self.credentials.user, Some(&self.credentials.password))
    }

    pub async fn api_get<T>(&self, path: &str) -> Result<T>
    where T: DeserializeOwned{
        Ok(self.authenticated_request(reqwest::Method::GET, path)
            .send()
            .await?
            .json::<OcsWrapper<T>>()
            .await?
            .ocs
            )
    }

    pub async fn capabilities(&self) -> ApiResult<CapabilityData> {
        self.api_get("/ocs/v1.php/cloud/capabilities").await
    }
}

#[derive(Deserialize, Debug)]
pub struct ApiResponseMeta {
    pub status: String,
    pub statuscode: u32,
    pub message: String,
    pub totalitems: String,
    pub itemsperpage: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<D> {
    pub meta: ApiResponseMeta,
    pub data: D
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionData {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
    pub string: String,
    pub edition: String,
    pub extended_support: bool,
}

#[derive(Deserialize, Debug)]
pub struct CapabilityData {
    pub version: VersionData,
    pub capabilities: std::collections::HashMap<String, Value>,
}

impl CapabilityData {
    pub fn get_capabilities<T>(&self, key: &str) -> Result<T>
    where T: DeserializeOwned {
        self.capabilities.get(key)
            .ok_or(NextcloudApiError::MissingCapabilityError)
            .and_then(|raw| serde_json::from_value(raw.clone()).map_err(Into::into))
    }
}
