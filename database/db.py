from boto3 import resource
from botocore.exceptions import ClientError
from os import getenv
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Initialize DynamoDB resource
dynamodb = resource("dynamodb",
        aws_access_key_id=getenv("ACCESS_KEY"),
        aws_secret_access_key=getenv("SECRET_KEY"),
        region_name=getenv("REGION_NAME"))

# Define tables
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
        ],
    },
    {
        "TableName": "Item",
        "AttributeDefinitions": [
            {
                'AttributeName': 'id',
                'AttributeType': 'S'
            },
        ],
        "KeySchema": [
            {
                'AttributeName': 'id',
                'KeyType': 'HASH'
            }
        ]
    },
    {
        "TableName": "Order",
        "AttributeDefinitions": [
            {
                'AttributeName': 'order_id',
                'AttributeType': 'S'
            },
            {
                'AttributeName': 'customer_id',
                'AttributeType': 'S'
            },
        ],
        "KeySchema": [
            {
                'AttributeName': 'order_id',
                'KeyType': 'HASH'
            }
        ],
        "GlobalSecondaryIndexes": [
            {
                'IndexName': 'CustomerOrderIndex',
                'KeySchema': [
                    {
                        'AttributeName': 'customer_id',
                        'KeyType': 'HASH'
                    },
                    {
                        'AttributeName': 'order_id',
                        'KeyType': 'RANGE'
                    }
                ],
                'Projection': {
                    'ProjectionType': 'ALL'
                },
            }
        ]
    }
]

def create_tables():
    for table in tables:
        try:
            # Check if the table already exists
            existing_table = dynamodb.Table(table["TableName"])
            existing_table.load()
            print(f"Table {table['TableName']} already exists.")
        except ClientError as e:
            if e.response['Error']['Code'] == 'ResourceNotFoundException':
                # Table does not exist, so create it
                create_table_params = {
                    "TableName": table["TableName"],
                    "KeySchema": table["KeySchema"],
                    "AttributeDefinitions": table["AttributeDefinitions"],
                    "BillingMode": "PAY_PER_REQUEST",
                }

                # Only add GlobalSecondaryIndexes if they exist and are non-empty
                if "GlobalSecondaryIndexes" in table and table["GlobalSecondaryIndexes"]:
                    create_table_params["GlobalSecondaryIndexes"] = table["GlobalSecondaryIndexes"]

                dynamodb.create_table(**create_table_params)
                print(f"Table {table['TableName']} created successfully.")
            else:
                print(f"Unexpected error: {e}")
                raise
        except Exception as e:
            print(f"Error while checking/creating table {table['TableName']}: {e}")
            raise

if __name__ == "__main__":
    create_tables()
