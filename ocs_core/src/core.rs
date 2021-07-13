//! Core Nextcloud OCS API implementation

use crate::{NextcloudApiClient, Result, ApiResult, NextcloudApiError};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use async_trait::async_trait;

#[async_trait]
pub trait OcsCoreApi {
    /// Get the server's capabilities
    async fn capabilities(&self) -> ApiResult<CapabilityData>;
    /// Get the metadata for a single user
    async fn user_metadata(&self, user_id: &str) -> ApiResult<UserMetadata>;
    /// Get the metadata for the API user
    async fn own_metadata(&self) -> ApiResult<UserMetadata>;
    /// Get a list of all users (Only available to admins)
    async fn users(&self) -> ApiResult<UserList>;
}

#[async_trait]
impl OcsCoreApi for NextcloudApiClient {
    async fn capabilities(&self) -> ApiResult<CapabilityData> {
        self.api_get("/ocs/v1.php/cloud/capabilities").await
    }

    async fn user_metadata(&self, user_id: &str) -> ApiResult<UserMetadata> {
        let url = format!("/ocs/v1.php/cloud/users/{}", user_id);
        self.api_get(&url).await
    }

    async fn own_metadata(&self) -> ApiResult<UserMetadata> {
        self.user_metadata(&self.credentials.user).await
    }

    async fn users(&self) -> ApiResult<UserList> {
        self.api_get("/ocs/v1.php/cloud/users").await
    }
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

#[derive(Deserialize, Debug)]
pub struct QuotaData {
    pub free: i64,
    pub used: i64,
    pub total: i64,
    pub relative: f32,
    pub quota: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserMetadata {
    pub enabled: bool,
    pub storage_location: String,
    pub id: String,
    pub last_login: u64,
    pub backend: String,
    pub subadmin: Value,
    pub quota: QuotaData,
    pub email: Option<String>,
    pub displayname: String,
    pub phone: String,
    pub address: String,
    pub website: String,
    pub twitter: String,
    pub groups: Vec<String>,
    pub language: String,
    pub locale: String,
    pub backend_capabilities: HashMap<String, bool>,
}

#[derive(Deserialize, Debug)]
pub struct UserList {
    pub users: Vec<String>,
}
