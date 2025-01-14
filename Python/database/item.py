from decimal import Decimal
import decimal
from uuid import uuid4
from aiodynamo.errors import ItemNotFound
from boto3.dynamodb.types import TypeDeserializer, TypeSerializer
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from models.item import Item
from .db import DynamoDBClient

dynamo = DynamoDBClient()
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

async def create_item(item: dict):

    try:
        async with DynamoDBClient() as client:
            item['price'] = Decimal(str(item['price']))
            item['id'] = str(uuid4())
            
            dynamo_item = to_dynamodb_json(item)

            await client.put_item(TableName="Item",Item = dynamo_item)
            return JSONResponse(content="Item created successfully!", status_code=200)
    except ClientError as e:
        return JSONResponse(content=e.response["error"], status_code=500)

async def get_items():
    try:
        async with DynamoDBClient() as client:
            
            items = await client.scan(TableName="Item", Limit=100)
            
            deserialized_items = deserialize(items['Items'])
            
            response = [Item(**item) for item in deserialized_items]

            return JSONResponse(content = [item.dict() for item in response], status_code=200)

    except Exception as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)

async def get_item(id: str):
    try:
        async with DynamoDBClient() as client:   
            response = await client.get_item(
                TableName="Item",
                Key={"id": serializer.serialize(id)}
            )
        
            response = deserialize(response['Item'])

            return JSONResponse(content=response, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Item with ID: {id} does NOT exist."}, status_code=404)