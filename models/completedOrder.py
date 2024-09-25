from typing import Optional, List
from pydantic import BaseModel
from models.item import Item


class CompleteOrderResponse(BaseModel):
    order_id: str
    status: str
    items: List[Item]
    total_price: float
    final_price: float
    coupon_code: Optional[str] = None