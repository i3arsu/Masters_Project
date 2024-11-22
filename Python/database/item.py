from models.item import Item
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from aiodynamo.errors import ItemNotFound
from decimal import Decimal
from .db import dynamo_client
from uuid import uuid4



async def create_item(item: dict):
    table = dynamo_client.client.table("Item")

    try:
        item['price'] = Decimal(str(item['price']))
        item['id'] = str(uuid4())

        await table.put_item(item = item)
        return item
    except ClientError as e:
        return JSONResponse(content=e.response["error"], status_code=500)

async def get_items():
    table = dynamo_client.client.table("Item")

    try:
        items = []
        # Iterate over the asynchronous generator
        async for item in table.scan(limit=200):
            items.append(item)

        # Convert the items to the Item Pydantic model
        return [Item(**item) for item in items]

    except Exception as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)

async def get_item(id: str):
    table = dynamo_client.client.table("Item")
    
    try:
        response = await table.get_item(
            key={"id": id}
        )

        return JSONResponse(content=response, status_code=200)
    
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    
    except ItemNotFound as e:
        return JSONResponse(content={"error": f"Item with ID: {id} does NOT exist."}, status_code=404)