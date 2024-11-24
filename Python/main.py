import uvicorn
from fastapi import FastAPI
from pathlib import Path
from mangum import Mangum
from routes.item import item_router
from routes.order import order_router
from routes.coupon import coupon_router


app = FastAPI()
handler = Mangum(app)

app.include_router(item_router, prefix="/item")
app.include_router(coupon_router, prefix="/coupon")
app.include_router(order_router, prefix="/order")


if __name__ == "__main__":
    uvicorn.run(f"{Path(__file__).stem}:app", host="0.0.0.0", port=8000, env_file=".env")