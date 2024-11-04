import datetime
import json
from models.coupon import Coupon
from .db import dynamodb
from fastapi import HTTPException
from decimal import Decimal
from models.order import OrderItem, OrderRequest, OrderResponse
from .coupon import get_coupon
from .item import get_item
from typing import Optional, List
from uuid import uuid4

table = dynamodb.Table("Order")

def calculatePrice(order: OrderRequest, coupon: Optional[Coupon], fetched_items: List[dict]) -> float:
    total_price = sum(item['price'] * order_item.quantity for item, order_item in zip(fetched_items, order.items))
    
    if not coupon:
        return total_price  # No coupon, return total price

    # Check if the coupon is expired
    if coupon.expires_at and datetime.datetime.fromisoformat(coupon.expires_at) < datetime.datetime.now():
        raise HTTPException(status_code=400, detail="Coupon has expired")

    discount = 0.0
    print(coupon.discount_percentage)

    # Calculate the discount
    if coupon.applicable_items:
        # Discount only on specific items
        for item, order_item in zip(fetched_items, order.items):
            if item['id'] in coupon.applicable_items:
                discount += item['price'] * order_item.quantity * (coupon.discount_percentage / 100)
    else:
        # General discount on total price
        discount = total_price * (coupon.discount_percentage / 100)

    final_price = max(0.0, total_price - discount)  # Final price should not be negative
    return final_price

def applyCoupon(order: OrderRequest):
    coupon = None
    if order.coupon_code:
        response = get_coupon(order.coupon_code)
        
        # Check if the response contains valid coupon data
        if response.status_code == 200:  # Adjust based on your response's success code
            response_data = json.loads(response.body.decode())
            coupon = Coupon(**response_data)
        else:
            # Handle coupon not found case
            raise HTTPException(status_code=400, detail="Coupon not found")

    fetched_items = []
    for item in order.items:
        item_response = get_item(item.item_id)
        item_data = json.loads(item_response.body.decode())
        fetched_items.append(item_data)

    total_price = sum(item['price'] * order_item.quantity for item, order_item in zip(fetched_items, order.items))
    final_price = calculatePrice(order, coupon, fetched_items)

    return {
        "order_id": str(uuid4()),
        "total_price": total_price,
        "final_price": final_price,
        "discount_applied": bool(coupon),
    }

