from typing import List, Optional
from pydantic import BaseModel

from .item import Item

class OrderRequest(BaseModel):
    items: List[Item]
    coupon_code: Optional[str] = None
