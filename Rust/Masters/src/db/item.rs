use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Error, Client};
use crate::models::item::Item;
use crate::db::dynamodb::get_dynamodb_client;
use actix_web::{web, HttpResponse, Responder};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("DynamoDB error: {0}")]
    DynamoDbError(#[from] aws_sdk_dynamodb::Error),

    #[error("Failed to parse attribute")]
    ParseError,
}

pub async fn get_item_by_id(id: &str) -> Result<Item, ItemError> {
    let client = get_dynamodb_client().await;

    let request = client
        .get_item()
        .table_name("Item")
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|err| {
            ItemError::DynamoDbError(err.into())
        })?;

    if let Some(item) = request.item {
        let fetched_item = Item {
            id: item["id"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
            name: item["name"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
            price: item["price"].as_n().map_err(|_| ItemError::ParseError)?.parse().map_err(|_| ItemError::ParseError)?,
        };
        Ok(fetched_item)
    } else {
        Err(ItemError::NotFound("Item not found".to_string()))
    }
}
