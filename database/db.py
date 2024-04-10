from boto3 import resource
from os import getenv
from dotenv import load_dotenv, dotenv_values

load_dotenv() 

dynamodb = resource("dynamodb",
        aws_access_key_id=getenv("ACCESS_KEY"),
        aws_secret_access_key=getenv("SECRET_KEY"),
        region_name=getenv("REGION_NAME"))


# tables = [
#     {
#         "TableName": "Item",
        
#         "AttributeDefinitions": [
#             {
#                 'AttributeName': 'barcode',
#                 'AttributeType': 'S'
#             },
#             {
#                 'AttributeName': 'name',
#                 'AttributeType': 'S'
#             },
#             {
#                 'AttributeName': 'price',
#                 'AttributeType': 'N'
#             },
#             {
#                 'AttributeName': 'quantity',
#                 'AttributeType': 'N'
#             },
#             {
#                 'AttributeName': 'category',
#                 'AttributeType': 'S'
#             }
#         ],

#         "KeySchema": [
#             {
#                 'AttributeName': 'barcode',
#                 'KeyType': 'HASH'
#             }
#         ]
#     },
#     {
#         "TableName": "Coupon",
        
#         "AttributeDefinitions": [
#             {
#                 'AttributeName': 'code',
#                 'AttributeType': 'S'
#             },
#             {
#                 'AttributeName': 'discount_percent',
#                 'AttributeType': 'N'
#             },
#             {
#                 'AttributeName': 'applicable_items',
#                 'AttributeType': 'SS'  # Assuming 'applicable_items' is a set of strings
#             }
#         ],

#         "KeySchema": [
#             {
#                 'AttributeName': 'code',
#                 'KeyType': 'HASH'
#             }
#         ]
#     }
# ]

tables = [
  {
        "TableName": "Coupon",
        
        "AttributeDefinitions": [
            {
                'AttributeName': 'code',
                'AttributeType': 'S'
            }
        ],

        "KeySchema": [
            {
                'AttributeName': 'code',
                'KeyType': 'HASH'
            }
        ]
    },
    {
        "TableName": "Item",
        
        "AttributeDefinitions": [
            {
                'AttributeName': 'barcode',
                'AttributeType': 'S'
            }
        ],

        "KeySchema": [
            {
                'AttributeName': 'barcode',
                'KeyType': 'HASH'
            }
        ]
    }
]




def create_tables():
    try:
        for table in tables:
            dynamodb.create_table(
                TableName=table["TableName"],
                KeySchema=table["KeySchema"],
                AttributeDefinitions=table["AttributeDefinitions"],
                BillingMode="PAY_PER_REQUEST"
            )
    except Exception as e:

        print(e)