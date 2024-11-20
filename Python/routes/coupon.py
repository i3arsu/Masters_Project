from fastapi import APIRouter
from database.coupon import create_coupon, get_all, get_coupon
from models.coupon import Coupon

coupon_router = APIRouter()

@coupon_router.post("/create", response_model=Coupon)
async def create(coupon: Coupon):
    response = await create_coupon(coupon.model_dump())
    return response

@coupon_router.get("/getall")
async def getAll():
    response = await get_all()
    return response

@coupon_router.get("/get/{code}", response_model=Coupon)
async def getCoupon(code: str):
    response = await get_coupon(code)
    return response