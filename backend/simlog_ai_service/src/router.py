from fastapi import FastAPI
from fastapi.routing import APIRouter

router = APIRouter()

@router.get("/query")
def query():
    return "You are asking ai"