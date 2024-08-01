from fastapi import APIRouter
from models.orderRequest import OrderRequest
from models.orderResponse import OrderResponse
from database.coupon import applyCoupon, completeOrder
from models.coupon import Coupon

coupon_router = APIRouter()

@coupon_router.post("/apply_coupon/", response_model=OrderResponse)
async def apply(order: OrderRequest):
    return applyCoupon(order.model_dump())