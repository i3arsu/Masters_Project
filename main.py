import uvicorn
from fastapi import FastAPI
from pathlib import Path

if __name__ == "__main__":
    uvicorn.run(f"{Path(__file__).stem}:app", host="0.0.0.0", port=8000, debug=True, env_file=".env")