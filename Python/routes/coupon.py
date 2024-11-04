from fastapi import APIRouter
from database.coupon import create_coupon, get_all, get_coupon
from models.coupon import Coupon

coupon_router = APIRouter()

@coupon_router.post("/create", response_model=Coupon)
async def create(coupon: Coupon):
    return create_coupon(coupon.model_dump())

@coupon_router.get("/getall")
async def getAll():
    return get_all()

@coupon_router.get("/get/{code}", response_model=Coupon)
async def getCoupon(code: str):
    return get_coupon(code)