import aioboto3

class DynamoDBTables:
    _client = None
    _tables = {}

    @classmethod
    async def get_resource(cls):
        session = aioboto3.Session()
        if cls._client is None:
            async with session.client("dynamodb", region_name="eu-north-1") as dynamo_client:
                cls._client = dynamo_client
        return cls._client

    @classmethod
    async def get_table(cls, table_name: str):
        if table_name not in cls._tables:
            client = await cls.get_resource()
            cls._tables[table_name] = await client.Table(table_name)
        return cls._tables[table_name]