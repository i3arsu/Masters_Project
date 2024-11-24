import aioboto3

class DynamoDBTables:
    _resource = None
    _tables = {}

    @classmethod
    async def get_resource(cls):
        session = aioboto3.Session()
        if cls._resource is None:
            async with session.resource("dynamodb", region_name="eu-north-1") as dynamo_resource:
                cls._resource = dynamo_resource
        return cls._resource

    @classmethod
    async def get_table(cls, table_name: str):
        if table_name not in cls._tables:
            resource = await cls.get_resource()
            cls._tables[table_name] = await resource.Table(table_name)
        return cls._tables[table_name]