from decimal import Decimal
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import dynamodb
from fastapi import HTTPException
from models.coupon import Coupon

table = dynamodb.Table("Coupon")

def decimal_to_float(obj):
    if isinstance(obj, list):
        return [decimal_to_float(i) for i in obj]
    elif isinstance(obj, dict):
        return {k: decimal_to_float(v) for k, v in obj.items()}
    elif isinstance(obj, Decimal):
        return float(obj)
    else:
        return obj

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

        coupon = response.get('Item')
        coupon = decimal_to_float(coupon)

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