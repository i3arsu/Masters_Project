import datetime

from decimal import Decimal
from typing import Optional
from uuid import uuid4
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import dynamodb
from fastapi import HTTPException
from models.coupon import Coupon
from models.orderRequest import OrderRequest

table = dynamodb.Table("Coupon")
order_table = dynamodb.Table('Order')


def calculatePrice(order: OrderRequest, coupon: Optional[Coupon]) -> float:
    if not coupon:
        return sum(item.price for item in order.items)  # No coupon, return total price

    # Check if the coupon is expired
    if coupon.expires_at and datetime.datetime.fromisoformat(coupon.expires_at) < datetime.datetime.now():
        raise HTTPException(status_code=400, detail="Coupon has expired")

    discount = 0.0
    total_price = sum(item['price'] for item in order['items'])

    # Calculate the discount
    if coupon.applicable_items:
        # Discount only on specific items
        for item in order.items:
            if item.id in coupon.applicable_items:
                discount += item.price * (coupon.discount_percentage / 100)
    else:
        # General discount on total price
        discount = total_price * (coupon.discount_percentage / 100)

    final_price = max(0.0, total_price - discount)  # Final price should not be negative
    return final_price

def applyCoupon(order: OrderRequest):
    coupon = None
    if order['coupon_code']:
        response = table.get_item(Key={'code': order['coupon_code']})
        if "Item" not in response:
            raise HTTPException(status_code=400, detail = "Coupon not found!")
        coupon = Coupon(**response['Item'])

    totalPrice = sum(item['price'] for item in order['items'])
    finalPrice = calculatePrice(order, coupon)

    return {
        "order_id": str(uuid4()),
        "total_price": totalPrice,
        "final_price": finalPrice,
        "coupon_code": order['coupon_code']
    }

def completeOrder(order: OrderRequest):  # Expecting order to be a dictionary
    coupon = None
    print(order)
    
    # Access coupon_code as a dictionary key
    if order['coupon_code']:
        response = table.get_item(Key={'code': order['coupon_code']})
        if 'Item' not in response:
            raise HTTPException(status_code=400, detail="Coupon not found")
        coupon = Coupon(**response['Item'])

    # Convert all float values to Decimal
    total_price = sum(Decimal(str(item['price'])) for item in order['items'])
    final_price = applyCoupon(order)
    final_price_decimal = Decimal(str(final_price['final_price']))

    for item in order['items']:
        item['price'] = Decimal(str(item['price']))

    # Generate order ID
    order_id = str(uuid4())

    print(type(final_price_decimal), type(total_price))

    # Store order in the DB
    order_data = {
        'order_id': order_id,
        'items': order['items'],
        "total_price": Decimal(total_price),  # Ensure total_price is Decimal
        "final_price": final_price_decimal,   # Ensure final_price is Decimal
        'coupon_code': order.get('coupon_code'),
        'status': 'completed'
    }
    order_table.put_item(Item=order_data)

    return {
        "order_id": order_id,
        "status": 'completed',
        "total_price": float(total_price),  # Convert back to float if needed for response
        "final_price": float(final_price_decimal),  # Convert back to float for response
        "coupon_code": order.get('coupon_code'),
        "items": order['items']
    }


def create_coupon(coupon: Coupon):
    try:
        # Convert the coupon model to a dictionary
        coupon_data = coupon

        # Convert discount_percentage to Decimal if it's provided
        if coupon_data.get('discount_percentage') is not None:
            coupon_data['discount_percentage'] = Decimal(str(coupon_data['discount_percentage']))
        
        # Save the coupon to the DynamoDB table
        table.put_item(Item=coupon_data)
        
        return coupon

    except ClientError as e:
        raise HTTPException(status_code=500, detail=str(e))
    
def get_coupon(code: str):
    try:
        response = table.get_item(Key={"code": code})

        coupon = response.get('Coupon')
        
        if not coupon:
            return JSONResponse(content={"message": "Coupon not found"}, status_code=404)
        
        return JSONResponse(content=coupon, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    
def get_all():
    try:
        response = table.scan(Limit=200)

        coupons = response.get("Items", [])

        return [Coupon(**coupon) for coupon in coupons]

    except ClientError as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)