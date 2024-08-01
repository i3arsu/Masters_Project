from pydantic import BaseModel, Field
from uuid import uuid4

class Item(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid4()))  # Unique ID for the item
    name: str
    price: float