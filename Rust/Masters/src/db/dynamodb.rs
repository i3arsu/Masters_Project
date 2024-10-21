extern crate aws_sdk_dynamodb;
extern crate aws_types;
extern crate tokio;

use aws_sdk_dynamodb::{Client, Config};
use aws_types::SdkConfig;
use std::sync::Arc;
use tokio::sync::OnceCell;


// Singleton
static DYNAMODB_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn get_dynamodb_client() -> Arc<Client> {
    DYNAMODB_CLIENT
        .get_or_init(|| async {
            let shared_config = aws_config::load_from_env().await;
            let client = Client::new(&shared_config);
            Arc::new(client)
        })
        .await
        .clone()
}
