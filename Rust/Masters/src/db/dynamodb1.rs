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
    let mut update_expression = String::from("SET is_redeemed = :r");
    let mut expression_values = std::collections::HashMap::new();
    expression_values.insert(":r".to_string(), AttributeValue::Bool(coupon.is_redeemed));

    // Only add discount_percentage to update if it exists
    if let Some(discount_percentage) = coupon.discount_percentage {
        update_expression.push_str(", discount_percentage = :dp");
        expression_values.insert(":dp".to_string(), AttributeValue::N(discount_percentage.to_string()));
    }

    // Only add applicable_items to update if they exist
    if let Some(applicable_items) = coupon.applicable_items {
        update_expression.push_str(", applicable_items = :ai");
        expression_values.insert(":ai".to_string(), AttributeValue::L(
            applicable_items.into_iter().map(|item| AttributeValue::S(item)).collect()
        ));
    }

    // Only add expires_at to update if it exists
    if let Some(expires_at) = coupon.expires_at {
        update_expression.push_str(", expires_at = :ea");
        expression_values.insert(":ea".to_string(), AttributeValue::S(expires_at));
    }

    client
        .update_item()
        .table_name("Coupons")
        .key("code", AttributeValue::S(coupon.code.clone()))
        .update_expression(&update_expression)
        .expression_attribute_values(expression_values)
        .send()
        .await?;

    Ok(())
}
