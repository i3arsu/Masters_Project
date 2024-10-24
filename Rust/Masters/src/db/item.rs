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

pub async fn get_items_by_ids(client: &Client, ids: &[String]) -> Result<Vec<Item>, ItemError> {
    let mut items = Vec::new();

    for id in ids {
        let request = client
            .get_item()
            .table_name("Item")
            .key("id", AttributeValue::S(id.clone()))
            .send()
            .await
            .map_err(|err| ItemError::DynamoDbError(err.into()))?;

        if let Some(item) = request.item {
            let fetched_item = Item {
                id: item["id"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
                name: item["name"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
                price: item["price"].as_n().map_err(|_| ItemError::ParseError)?.parse().map_err(|_| ItemError::ParseError)?,
            };
            items.push(fetched_item);
        }
    }

    Ok(items)
}

pub async fn get_all_items() -> Result<Vec<Item>, ItemError> {
    let client = get_dynamodb_client().await;

    // Perform a scan operation to retrieve all items from the table
    let request = client
        .scan()
        .table_name("Item")
        .send()
        .await
        .map_err(|err| {
            ItemError::DynamoDbError(err.into())
        })?;

    // Extract the items from the response
    let items = request.items.unwrap_or_default();

    // Convert the items into a Vec<Item>
    let fetched_items: Result<Vec<Item>, ItemError> = items.into_iter().map(|item| {
        let id = item.get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or(ItemError::ParseError)?
            .to_string();

        let name = item.get("name")
            .and_then(|v| v.as_s().ok())
            .ok_or(ItemError::ParseError)?
            .to_string();

        let price = item.get("price")
            .and_then(|v| v.as_n().ok())
            .ok_or(ItemError::ParseError)?
            .parse::<f64>()
            .map_err(|_| ItemError::ParseError)?;

        Ok(Item { id, name, price })
    }).collect();

    Ok(fetched_items?)
}
