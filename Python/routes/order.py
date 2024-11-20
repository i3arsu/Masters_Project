from fastapi import APIRouter
from models.order import OrderRequest, OrderResponse
from database.order import applyCoupon

order_router = APIRouter()

@order_router.post("/apply/", response_model=OrderResponse)
async def apply(order: OrderRequest):
    response = await applyCoupon(order)
    return response