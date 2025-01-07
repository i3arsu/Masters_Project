from models.item import Item
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from aiodynamo.errors import ItemNotFound
from boto3.dynamodb.conditions import Key
from decimal import Decimal
from .db import DynamoDBTables
from uuid import uuid4

def decimal_to_float(obj):
    if isinstance(obj, list):
        return [decimal_to_float(i) for i in obj]
    elif isinstance(obj, dict):
        return {k: decimal_to_float(v) for k, v in obj.items()}
    elif isinstance(obj, Decimal):
        return float(obj)
    else:
        return obj

async def create_item(item: dict):
    table = await DynamoDBTables.get_table("Item")

    try:
        item['price'] = Decimal(str(item['price']))
        item['id'] = str(uuid4())

        await table.put_item(Item = item)
        return JSONResponse(content=item, status_code=200)
    except ClientError as e:
        return JSONResponse(content=e.response["error"], status_code=500)

async def get_items():
    table = await DynamoDBTables.get_table("Item")
    
    try:
    
        items = await table.scan(Limit=100)
        
        response = [Item(**item) for item in items['Items']]

        return JSONResponse(content = [item.dict() for item in response], status_code=200)

    except Exception as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)

async def get_item(id: str):
    table = await DynamoDBTables.get_table("Item")

    try:
        response = await table.get_item(
            Key={"id": id}
        )
        
        response = decimal_to_float(response['Item'])

        return JSONResponse(content=response, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Item with ID: {id} does NOT exist."}, status_code=404)