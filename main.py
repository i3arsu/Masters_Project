import uvicorn
from fastapi import FastAPI
from pathlib import Path
from mangum import Mangum
from database.db import create_tables
from routes.item import item_router


app = FastAPI()
handler = Mangum(app)

app.include_router(item_router, prefix="/item")

if __name__ == "__main__":
    try:
        print("Creating Tables:")
        create_tables()
    except BaseException as e:
        print(e)

    uvicorn.run(f"{Path(__file__).stem}:app", host="0.0.0.0", port=8000, env_file=".env")