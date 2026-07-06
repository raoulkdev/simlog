from fastapi.routing import APIRouter
from models import FlightQueryPayload
from handlers import query_ai

router = APIRouter()

@router.post("/query_ai")
def query(flight_query_payload: FlightQueryPayload):
    return query_ai(flight_query_payload)