use aws_sdk_dynamodb::{Client, Error};
use aws_config::meta::region::RegionProviderChain;

pub async fn get_dynamodb_client() -> Result<Client, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    Ok(client)
}