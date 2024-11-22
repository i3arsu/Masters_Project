import os
from aiodynamo.client import Client
from aiodynamo.credentials import Credentials
from aiodynamo.http.aiohttp import AIOHTTP
import aiohttp

class DynamoDBClient:
    def __init__(self):
        self.client = None
        self.session = None

    async def init_client(self):

        self.session = aiohttp.ClientSession()
        self.client = Client(
            AIOHTTP(self.session),
            Credentials.auto(),
            "eu-north-1",
        )

    async def close_client(self):
        if self.session:
            await self.session.close()

# Create a shared DynamoDB client instance
dynamo_client = DynamoDBClient()