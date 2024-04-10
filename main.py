import uvicorn
from fastapi import FastAPI
from pathlib import Path
from mangum import Mangum
from database.db import create_tables


app = FastAPI()
handler = Mangum(app)

create_tables()

if __name__ == "__main__":
    uvicorn.run(f"{Path(__file__).stem}:app", host="0.0.0.0", port=8000, env_file=".env")