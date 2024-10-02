use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::model::AttributeValue;
use decimal::Decimal;
use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, HttpException};
use crate::models::Item;

// Converts DynamoDB Decimal fields to f64 for easy handling
fn decimal_to_float(obj: &AttributeValue) -> f64 {
    match obj {
        AttributeValue::N(num) => num.parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

pub async fn create_item(client: &Client, item: &mut Item) -> Result<Item, HttpException> {
    item.price = Decimal::from_f64(item.price).unwrap();

    let item_data = item.clone();

    client.put_item().table_name("Item").item(item_data).send().await?;
    
    Ok(item.clone())
}

pub async fn get_item(client: &Client, id: &str) -> Result<HttpResponse, HttpException> {
    let response = client
        .get_item()
        .table_name("Item")
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await?;

    let item = response.item;

    if item.is_none() {
        return Err(HttpException::new(404, "Item not found"));
    }

    let item = item.unwrap();
    let item = decimal_to_float(&item);

    Ok(HttpResponse::Ok().json(item))
}

pub async fn get_items(client: &Client) -> Result<Vec<Item>, HttpException> {
    let response = client.scan().table_name("Item").limit(200).send().await?;

    let items: Vec<Item> = response.items().unwrap_or_default()
        .iter()
        .map(|item| Item::from(item.clone()))
        .collect();

    Ok(items)
}

pub async fn delete_item(client: &Client, barcode: &str) -> Result<HttpResponse, HttpException> {
    let response = client.delete_item()
        .table_name("Item")
        .key("barcode", AttributeValue::S(barcode.to_string()))
        .send()
        .await?;

    Ok(HttpResponse::Ok().json(response))
}
