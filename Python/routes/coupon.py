from fastapi import APIRouter
from models.orderRequest import OrderRequest
from models.orderResponse import OrderResponse
from database.coupon import applyCoupon, create_coupon, get_all
from models.coupon import Coupon

coupon_router = APIRouter()

@coupon_router.post("/apply_coupon/", response_model=OrderResponse)
async def apply(order: OrderRequest):
    return applyCoupon(order)

@coupon_router.post("/create", response_model=Coupon)
async def create(coupon: Coupon):
    return create_coupon(coupon.model_dump())

@coupon_router.get("/all")
async def getAll():
    return get_all()