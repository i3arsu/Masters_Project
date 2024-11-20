from decimal import Decimal
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import dynamo_client
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

    table = dynamo_client.client.table("Coupon")

    try:
        coupon_data = coupon

        # Convert discount_percentage to Decimal if it's provided
        if coupon_data.get('discount_percentage') is not None:
            coupon_data['discount_percentage'] = Decimal(str(coupon_data['discount_percentage']))
        
        # Save the coupon to the DynamoDB table
        await table.put_item(Item=coupon_data)
        
        return coupon

    except ClientError as e:
        raise HTTPException(status_code=500, detail=str(e))
    
async def get_coupon(code: str):

    table = dynamo_client.client.table("Coupon")

    try:
        coupon = await table.get_item(key={"code": code})
        
        coupon = decimal_to_float(coupon)
        return JSONResponse(content=coupon, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Coupon: {code} does NOT exist."}, status_code=404)
    
async def get_all():

    table = dynamo_client.client.table("Coupon")

    try:
        coupons = []
        async for coupon in table.scan(limit=200):
            coupons.append(coupon)

        # Convert the items to the Item Pydantic model
        return [Coupon(**coupon) for coupon in coupons]

    except ClientError as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)