use nextcloud_ocs_core::{NextcloudApiClient, Result};
use async_trait::async_trait;



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
