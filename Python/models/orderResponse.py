from typing import Optional
from pydantic import BaseModel


class OrderResponse(BaseModel):
    order_id: str
    total_price: float
    final_price: float
    coupon_code: Optional[str] = None