from decimal import Decimal
from fastapi.responses import JSONResponse
from aiohttp import ClientError
from .db import DynamoDBClientManager
from boto3.dynamodb.types import TypeDeserializer, TypeSerializer
from fastapi import HTTPException
from aiodynamo.errors import ItemNotFound
from models.coupon import Coupon

serializer = TypeSerializer()
deserializer = TypeDeserializer()

def convert_decimal(obj):
    if isinstance(obj, list):
        return [convert_decimal(item) for item in obj]
    elif isinstance(obj, dict):
        return {k: convert_decimal(v) for k, v in obj.items()}
    elif isinstance(obj, Decimal):
        return int(obj) if obj % 1 == 0 else float(obj)
    else:
        return obj
    
def to_dynamodb_json(data):
    """
    Recursively serialize Python data to DynamoDB JSON format.
    """
    if isinstance(data, dict):
        # Serialize each key-value pair in the dictionary
        return {key: to_dynamodb_json(value) for key, value in data.items()}
    elif isinstance(data, list):
        # Serialize each element in the list
        return {"L": [to_dynamodb_json(item) for item in data]}
    else:
        # Use TypeSerializer for primitive types
        return serializer.serialize(data)

def deserialize(data):
    if isinstance(data, list):
        return [deserialize(v) for v in data]
    elif isinstance(data, dict):
        try:
            # Deserialize using TypeDeserializer
            deserialized_data = deserializer.deserialize(data)
        except TypeError:
            # Handle nested dictionaries
            deserialized_data = {k: deserialize(v) for k, v in data.items()}
        # Convert remaining Decimal values
        return convert_decimal(deserialized_data)
    else:
        return data
    
async def create_coupon(coupon: Coupon):
    client = await DynamoDBClientManager.get_client()
    try:
            
        coupon_data = coupon

        # Convert discount_percentage to Decimal if it's provided
        if coupon_data.get('discount_percentage') is not None:
            coupon_data['discount_percentage'] = Decimal(str(coupon_data['discount_percentage']))
            
        # Save the coupon to the DynamoDB table
        await client.put_item(TableName="Coupon", Item=to_dynamodb_json(coupon_data))
            
        return JSONResponse(content="Coupon created successfully!", status_code=200)

    except ClientError as e:
        raise HTTPException(status_code=500, detail=str(e))
    
async def get_coupon(code: str):
    client = await DynamoDBClientManager.get_client()
    
    try:
        response = await client.get_item(
            TableName="Coupon",
            Key={"code": serializer.serialize(code)})
        
        response = deserialize(response['Item'])
        return JSONResponse(content=response, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Coupon: {code} does NOT exist."}, status_code=404)
    
async def get_all():
    client = await DynamoDBClientManager.get_client()
    
    try:
        coupons = await client.scan(TableName="Coupon", Limit=100)
            
        deserialized_coupons = deserialize(coupons['Items'])
            
        response = [Coupon(**coupon) for coupon in deserialized_coupons]
            
        return JSONResponse(content=[coupon.dict() for coupon in response], status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)