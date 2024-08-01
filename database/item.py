from models.item import Item
from .db import dynamodb
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from boto3.dynamodb.conditions import Key
from decimal import Decimal

table = dynamodb.Table("Item")

def decimal_to_float(obj):
    if isinstance(obj, list):
        return [decimal_to_float(i) for i in obj]
    elif isinstance(obj, dict):
        return {k: decimal_to_float(v) for k, v in obj.items()}
    elif isinstance(obj, Decimal):
        return float(obj)
    else:
        return obj

def create_item(item: dict):

    try:
        item['price'] = Decimal(str(item['price']))

        table.put_item(Item = item)
        return item
    except ClientError as e:
        return JSONResponse(content=e.response["error"], status_code=500)
    
def get_item(id: str):
    try:
        response = table.query(
            KeyConditionExpression=Key("id").eq(id)
        )
        item = response.get('Item', [])
        if not item:
            return JSONResponse(content={"message": "Item not found"}, status_code=404)
        
        item = decimal_to_float(item)
        return JSONResponse(content=item, status_code=200)
    except ClientError as e:
        return JSONResponse(content={"error": e.response['Error']['Message']}, status_code=500)
    
def get_items():

    try:
        response = table.scan(Limit=200)

        # Correctly retrieve the items from the response
        items = response.get("Items", [])

        # Convert the items to the Item Pydantic model
        return [Item(**item) for item in items]

    except ClientError as e:
        return JSONResponse(content={"error": str(e)}, status_code=500)
    
def delete_item(item: dict):

    try:
        response = table.delete_item(
            Key={
                "barcode": item["barcode"]
            }
        )
        return response
    except ClientError as e:
        return JSONResponse(content=e.response["error"],status_code=500)