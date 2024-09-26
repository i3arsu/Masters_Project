from pydantic import BaseModel
from typing import List, Optional

class Coupon(BaseModel):
    code: str
    discount_percentage: Optional[float] = None
    applicable_items: Optional[List[str]] = None
    expires_at: Optional[str] = None