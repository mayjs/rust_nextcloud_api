use nextcloud_api::{NextcloudApiClient, NextcloudCredentials};
use nextcloud_api::core::OcsCoreApi;
use dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let creds = NextcloudCredentials::new(env::var("NC_USER").expect("User not set"), env::var("NC_PASS").expect("Pass not set"));
    let client = NextcloudApiClient::new(env::var("NC_URL").expect("URL not set"), creds).unwrap();

    let api_result = client.capabilities().await.unwrap();
    println!("Version: {:?}", api_result.data.version);
    println!("Capabilities: {:?}", api_result.data.capabilities);

    let user_meta = client.user_metadata(&env::var("NC_USER").unwrap()).await.unwrap();
    println!("Use Data: {:?}", user_meta.data);
    
    let all_users = client.users().await.unwrap();
    println!("Users: {:?}", all_users.data);
}
