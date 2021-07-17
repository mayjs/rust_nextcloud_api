use nextcloud_ocs_core::{NextcloudApiClient, NextcloudCredentials};
use nextcloud_ocs_core::core::OcsCoreApi;
use nextcloud_deck::DeckApi;
use dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let creds = NextcloudCredentials::new(env::var("NC_USER").expect("User not set"), env::var("NC_PASS").expect("Pass not set"));
    let client = NextcloudApiClient::new(env::var("NC_URL").expect("URL not set"), creds).unwrap();

    let res = client.boards(true).await.unwrap();
    println!("Boards: {:?}", res);
}
