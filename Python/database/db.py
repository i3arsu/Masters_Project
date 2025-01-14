import aioboto3

class DynamoDBClientManager:
    """
    A manager for creating and reusing a single aioboto3 DynamoDB client.
    """
    _session = None
    _client = None

    @staticmethod
    async def get_client():
        """
        Returns a reusable DynamoDB client.
        """
        if DynamoDBClientManager._client is None:
            if DynamoDBClientManager._session is None:
                DynamoDBClientManager._session = aioboto3.Session()
            DynamoDBClientManager._client = await DynamoDBClientManager._session.client("dynamodb").__aenter__()
        return DynamoDBClientManager._client

    @staticmethod
    async def close_client():
        """
        Closes the DynamoDB client to release resources.
        """
        if DynamoDBClientManager._client is not None:
            DynamoDBClientManager._client.__aexit__(None, None, None)
            DynamoDBClientManager._client = None