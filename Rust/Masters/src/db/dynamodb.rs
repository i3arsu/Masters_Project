use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::model::AttributeValue;
use crate::models::coupon::Coupon;
use serde_dynamodb;

pub async fn get_coupon_by_code(client: Client, code: String) -> Result<Option<Coupon>, Error> {
    let result = client
        .get_item()
        .table_name("Coupons")
        .key("code", AttributeValue::S(code))
        .send()
        .await?;

    if let Some(item) = result.item {
        let coupon: Coupon = serde_dynamodb::from_hashmap(item).unwrap();
        Ok(Some(coupon))
    } else {
        Ok(None)
    }
}

pub async fn update_coupon(coupon: Coupon, client: Client) -> Result<(), Error> {
    client
        .update_item()
        .table_name("Coupons")
        .key("code", AttributeValue::S(coupon.code.clone()))
        .update_expression("SET is_redeemed = :r")
        .expression_attribute_values(":r", AttributeValue::Bool(coupon.is_redeemed))
        .send()
        .await?;
    Ok(())
}
