from decimal import Decimal
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import DynamoDBTables
from fastapi import HTTPException
from aiodynamo.errors import ItemNotFound
from models.coupon import Coupon

def decimal_to_float(obj):
    if isinstance(obj, list):
        return [decimal_to_float(i) for i in obj]
    elif isinstance(obj, dict):
        return {k: decimal_to_float(v) for k, v in obj.items()}
    elif isinstance(obj, Decimal):
        return float(obj)
    else:
        return obj
    
async def create_coupon(coupon: Coupon):
    table = await DynamoDBTables.get_table("Coupon")

    try:
        coupon_data = coupon

        # Convert discount_percentage to Decimal if it's provided
        if coupon_data.get('discount_percentage') is not None:
            coupon_data['discount_percentage'] = Decimal(str(coupon_data['discount_percentage']))
        
        # Save the coupon to the DynamoDB table
        await table.put_item(Item=coupon_data)
        
        return JSONResponse(content=coupon_data, status_code=200)

    except ClientError as e:
        raise HTTPException(status_code=500, detail=str(e))
    
async def get_coupon(code: str):
    table = await DynamoDBTables.get_table("Coupon")

    try:
        coupon = await table.get_item(Key={"code": code})
        
        coupon = decimal_to_float(coupon)
        return JSONResponse(content=coupon, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Coupon: {code} does NOT exist."}, status_code=404)
    
async def get_all():
    table = await DynamoDBTables.get_table("Coupon")

    try:
        coupons = await table.scan(Limit=200)
        
        response = [Coupon(**coupon) for coupon in coupons['Items']]
        
        return JSONResponse(content=[coupon.dict() for coupon in response], status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)