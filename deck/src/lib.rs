use nextcloud_ocs_core::NextcloudApiClient;
use async_trait::async_trait;
use thiserror::Error;
use serde::Deserialize;
use serde_json::Value;

macro_rules! deck_api_path {
    ($rel_path:literal) => {
        concat!("index.php/apps/deck/api/v1.0", $rel_path)
    }
}

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
    async fn boards(&self, details: bool) -> Result<Vec<Board>>;
}

#[async_trait]
impl DeckApi for NextcloudApiClient {
    async fn boards(&self, details: bool) -> Result<Vec<Board>> {
        const DETAILS_HEADERS: &[(&str, &str)] = &[("details","true")];
        let headers = match details {
            true => Some(DETAILS_HEADERS),
            false => None
        };
        self.api_get_ext(deck_api_path!("/boards"), None, headers).await
    }
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: i64,
    pub title: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Permissions {
    pub permission_edit: bool,
    pub permission_manage: bool,
    pub permission_read: bool,
    pub permission_share: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub title: String,
    pub owner: Value,
    pub color: String,
    pub archived: bool,
    pub labels: Vec<Label>,
    pub acl: Vec<Value>,
    pub permissions: Permissions,
    pub users: Vec<Value>,
    pub shared: usize,
    pub deleted_at: usize,
    pub id: u64,
    pub last_modified: u64,
    pub settings: Value,
}
