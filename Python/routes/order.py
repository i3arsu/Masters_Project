from fastapi import APIRouter
from models.order import OrderRequest, OrderResponse
from models.completedOrder import CompleteOrderResponse
from database.order import completeOrder, applyCoupon

order_router = APIRouter()

@order_router.post("/apply/", response_model=OrderResponse)
async def apply(order: OrderRequest):
    return applyCoupon(order)