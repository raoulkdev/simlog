from fastapi import FastAPI
from router import router
from dotenv import load_dotenv
import os

# Load .env variables
load_dotenv()

# Setup server
simlog_ai_service = FastAPI()

# Setup router
simlog_ai_service.include_router(router)

if __name__ == "__main__":
    import uvicorn

    # Get server address info
    host = os.getenv("AI_SERVICE_HOST")
    port = os.getenv("AI_SERVICE_PORT")
    
    if host is None or port is None:
        raise ValueError("AI_SERVICE_HOST and AI_SERVICE_PORT must be set")
    
    # Run server
    uvicorn.run("main:simlog_ai_service", host=host, port=int(port))
