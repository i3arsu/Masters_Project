from pydantic import BaseModel
from typing import List

class Coupon(BaseModel):
    code: str
    discount_percent: float
    applicable_items: List[str] = []