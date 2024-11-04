from typing import Optional, List
from pydantic import BaseModel

class OrderItem(BaseModel):
    item_id: str
    quantity: int

class OrderRequest(BaseModel):
    items: List[OrderItem]
    coupon_code: Optional[str]

class OrderResponse(BaseModel):
    order_id: str
    total_price: float
    discount_applied: bool
    final_price: float