use nextcloud_ocs_core::NextcloudApiClient;
use async_trait::async_trait;
use thiserror::Error;
use serde::Deserialize;

#[derive(Error, Debug, Deserialize)]
#[error("Code {}: {}", .status, .message)]
pub struct DeckApiErrorResponse {
    pub status: u32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DeckApiResult<T> {
    DeckApiError(DeckApiErrorResponse),
    DeckApiOk(T),
}

/// An error in the Deck context
#[derive(Debug, Error)]
pub enum DeckApiError {
    #[error("an error occured on the basic nextcloud API level")]
    LowerError(#[from] nextcloud_ocs_core::NextcloudApiError),
    #[error("the Deck API returned an error: {0}")]
    DeckError(#[from] DeckApiErrorResponse),
}

pub type Result<T> = std::result::Result<T, DeckApiError>;

impl<T> From<DeckApiResult<T>> for Result<T> {
    fn from(v: DeckApiResult<T>) -> Self {
        match v {
            DeckApiResult::DeckApiOk(t) => Ok(t),
            DeckApiResult::DeckApiError(e) => Err(e.into())
        }
    }
}

#[async_trait]
pub trait DeckApi {
    async fn boards(&self) -> Result<serde_json::Value>;
}

#[async_trait]
impl DeckApi for NextcloudApiClient {
    async fn boards(&self) -> Result<serde_json::Value> {
        self.api_get("index.php/apps/deck/api/v1.0/boards").await
    }
}
