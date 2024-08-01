from typing import Optional
from pydantic import BaseModel


class CompleteOrderResponse(BaseModel):
    order_id: str
    status: str
    total_price: float
    final_price: float
    coupon_code: Optional[str] = None