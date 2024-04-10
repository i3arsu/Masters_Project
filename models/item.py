from pydantic import BaseModel, Field
from random import randint

def generate_barcode_number(length):
    barcode_number = ''.join(str(randint(0, 9)) for _ in range(length))
    return barcode_number


class Item(BaseModel):
    name: str
    price: float
    quantity: int
    category: str

    barcode: str
    