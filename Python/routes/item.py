from fastapi import APIRouter
from database.item import create_item, get_item, get_items
from models.item import Item

item_router = APIRouter()

@item_router.post("/create", response_model=Item)
async def create(item: Item):
    return create_item(item.model_dump())

@item_router.get("/id/{id}")
async def get_one_item(id: str):
    return get_item(id)

@item_router.get("/all")
async def get_all_items():
    return get_items()