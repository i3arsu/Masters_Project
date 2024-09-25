from fastapi import APIRouter
from models.orderRequest import OrderRequest
from models.completedOrder import CompleteOrderResponse
from database.coupon import completeOrder

order_router = APIRouter()

@order_router.post("/finalize", response_model=CompleteOrderResponse)
async def finalize(order: OrderRequest):
    return completeOrder(order.model_dump())