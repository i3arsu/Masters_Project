use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Error, Client};
use crate::models::item::Item;
use crate::db::dynamodb::get_dynamodb_client;
use actix_web::{web, HttpResponse, Responder};
use crate::errors::errors::ItemError;
use thiserror::Error;
use uuid::Uuid;

pub async fn get_item_by_id(id: &str) -> Result<Item, ItemError> {
    let client = get_dynamodb_client().await;

    let request = client
        .get_item()
        .table_name("Item")
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|err| ItemError::DynamoDbError)?;

    if let Some(item) = request.item {
        let fetched_item = Item {
            id: item["id"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
            name: item["name"].as_s().map_err(|_| ItemError::ParseError)?.to_string(),
            price: item["price"].as_n().map_err(|_| ItemError::ParseError)?.parse().map_err(|_| ItemError::ParseError)?,
        };
        Ok(fetched_item)
    } else {
        Err(ItemError::NotFound)
    }
}

pub async fn get_items_by_ids(client: &aws_sdk_dynamodb::Client, ids: &[String]) -> Result<Vec<Item>, ItemError> {
    let mut items = Vec::new();

    for id in ids {
        let request = client
            .get_item()
            .table_name("Item")
            .key("id", AttributeValue::S(id.clone()))
            .send()
            .await
            .map_err(|err| ItemError::DynamoDbError)?;

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

    let request = client
        .scan()
        .table_name("Item")
        .send()
        .await
        .map_err(|err| ItemError::DynamoDbError)?;

    let items = request.items.unwrap_or_default();

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

    fetched_items
}

pub async fn create_item(item: web::Json<Item>) -> Result<String, ItemError> {
    let client = get_dynamodb_client().await;

    let mut new_item = item.into_inner();

    let generated_id = Uuid::new_v4().to_string();
    new_item.id = generated_id.clone();

    // Perform the put_item operation to insert the item into the table
    client
        .put_item()
        .table_name("Item")
        .item("id", AttributeValue::S(new_item.id.clone()))
        .item("name", AttributeValue::S(new_item.name.clone()))
        .item("price", AttributeValue::N(new_item.price.to_string()))
        .send()
        .await
        .map_err(|err| ItemError::DynamoDbError)?;

    // Return a success message
    Ok("Item created successfully.".to_string())
}

pub async fn delete_item_by_id(item_id: &str) -> Result<(), ItemError> {
    let client = get_dynamodb_client().await;

    let request = client
        .delete_item()
        .table_name("Item")
        .key("id", AttributeValue::S(item_id.to_string()))
        .send()
        .await;

    match request {
        Ok(_) => Ok(()),
        Err(_) => Err(ItemError::DynamoDbError),
    }
}