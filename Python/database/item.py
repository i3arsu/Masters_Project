from decimal import Decimal
from uuid import uuid4
from .db import DynamoDBClientManager
from aiodynamo.errors import ItemNotFound
from utils.dynamodb_utils import to_dynamodb_json, deserialize
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from models.item import Item
from boto3.dynamodb.types import TypeSerializer

serializer = TypeSerializer()

async def create_item(item: dict) -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        item['price'] = Decimal(str(item['price']))  
        item['id'] = str(uuid4())  # Generate unique ID
        dynamo_item = to_dynamodb_json(item)

        await client.put_item(TableName="Item", Item=dynamo_item)
        return JSONResponse(content="Item created successfully!", status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": f"AWS Client Error: {e.response.get('Error', {}).get('Message', 'Unknown error')}"}, status_code=500)

async def get_items() -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        # Scan for items with a limit
        response = await client.scan(TableName="Item", Limit=100)
        deserialized_items = deserialize(response.get('Items', []))

        # Convert to Item models
        items = [Item(**item) for item in deserialized_items]
        return JSONResponse(content=[item.dict() for item in items], status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": f"AWS Client Error: {str(e)}"}, status_code=500)

async def get_item(id: str) -> JSONResponse:
    client = await DynamoDBClientManager.get_client()
    try:
        response = await client.get_item(
            TableName="Item",
            Key={"id": serializer.serialize(id)}
        )

        if 'Item' not in response:
            raise ItemNotFound(f"Item with ID: {id} does NOT exist.")

        item_data = deserialize(response['Item'])
        return JSONResponse(content=item_data, status_code=200)

    except ClientError as e:
        return JSONResponse(content={"error": f"AWS Client Error: {e.response.get('Error', {}).get('Message', 'Unknown error')}"}, status_code=500)
    except ItemNotFound:
        return JSONResponse(content={"error": f"Item with ID: {id} does NOT exist."}, status_code=404)