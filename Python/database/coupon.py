from decimal import Decimal
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import DynamoDBClientManager
from utils.dynamodb_utils import to_dynamodb_json, deserialize
from fastapi import HTTPException
from aiodynamo.errors import ItemNotFound
from models.coupon import Coupon
from boto3.dynamodb.types import TypeSerializer

serializer = TypeSerializer()
    
async def create_coupon(coupon: Coupon) -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        coupon_data = coupon.dict()

        if coupon_data.get('discount_percentage') is not None:
            coupon_data['discount_percentage'] = Decimal(str(coupon_data['discount_percentage']))

        await client.put_item(TableName="Coupon", Item=to_dynamodb_json(coupon_data))
        return JSONResponse(content="Coupon created successfully!", status_code=200)

    except ClientError as e:
        raise HTTPException(status_code=500, detail=f"AWS Client Error: {e}")
    
async def get_coupon(code: str) -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        response = await client.get_item(
            TableName="Coupon",
            Key={"code": serializer.serialize(code)}
        )

        coupon_data = deserialize(response['Item'])
        return JSONResponse(content=coupon_data, status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": f"AWS Client Error: {e.response['Error']['Message']}"}, status_code=500)
    except ItemNotFound:
        return JSONResponse(content={"error": f"Coupon: {code} does NOT exist."}, status_code=404)
    
async def get_all() -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        # Retrieve all items (limited to 100)
        response = await client.scan(TableName="Coupon", Limit=100)
        deserialized_coupons = deserialize(response.get('Items', []))

        # Convert deserialized data into Coupon models
        coupons = [Coupon(**coupon) for coupon in deserialized_coupons]
        return JSONResponse(content=[coupon.dict() for coupon in coupons], status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": f"AWS Client Error: {str(e)}"}, status_code=500)