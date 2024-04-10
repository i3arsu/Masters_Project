from .db import dynamodb
from botocore.exceptions import ClientError
from fastapi.responses import JSONResponse
from boto3.dynamodb.conditions import Key

table = dynamodb.Table("Item")

def create_item(item: dict):

    try:
        table.put_item(Item=item)
        return item
    except ClientError as e:
        return JSONResponse(content=e.response["error"], status_code=500)
    
def get_item(barcode: str):
    try:
        response = table.query(
            KeyConditionExpression=Key("barcode").eq(barcode)
        )
    except ClientError as e:
        return JSONResponse(content=e.response["error"],status_code=500)
    
def get_items():

    try:
        response = table.scan(
            Limit=200,
            AttributesToGet=["name", "price", "quantity", "category", "barcode"]
        )

        return response["Items"]

    except ClientError as e:

        return JSONResponse(content=e.response["error"], status_code=500)
    
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