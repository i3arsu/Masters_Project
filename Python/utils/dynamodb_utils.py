from decimal import Decimal
from typing import Any
from boto3.dynamodb.types import TypeDeserializer, TypeSerializer

serializer = TypeSerializer()
deserializer = TypeDeserializer()

def convert_decimal(obj: Any) -> Any:
    if isinstance(obj, Decimal):
        return int(obj) if obj % 1 == 0 else float(obj)
    elif isinstance(obj, list):
        return [convert_decimal(item) for item in obj]
    elif isinstance(obj, dict):
        return {k: convert_decimal(v) for k, v in obj.items()}
    return obj

def to_dynamodb_json(data: Any) -> dict:
    if isinstance(data, dict):
        return {key: to_dynamodb_json(value) for key, value in data.items()}
    if isinstance(data, list):
        return {"L": [to_dynamodb_json(item) for item in data]}
    return serializer.serialize(data)

def deserialize(data: Any) -> Any:
    if isinstance(data, list):
        return [deserialize(item) for item in data]
    if isinstance(data, dict):
        try:
            return convert_decimal(deserializer.deserialize(data))
        except TypeError:
            return {k: deserialize(v) for k, v in data.items()}
    return data