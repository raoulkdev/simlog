from fastapi import FastAPI
from router import router
from dotenv import load_dotenv
import os

load_dotenv()

simlog_ai_service = FastAPI()
simlog_ai_service.include_router(router)

if __name__ == "__main__":
    import uvicorn
    host = os.getenv("AI_SERVICE_HOST")
    port = os.getenv("AI_SERVICE_PORT")

    if host is None or port is None:
        raise ValueError("AI_SERVICE_HOST and AI_SERVICE_PORT must be set")
    
    uvicorn.run("main:simlog_ai_service", host=host, port=int(port))
