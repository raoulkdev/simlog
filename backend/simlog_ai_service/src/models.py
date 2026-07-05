from uuid import UUID
import pydantic
from datetime import date
from typing import Optional

class FlightQueryPayload(pydantic.BaseModel):
    flight_id: UUID
    user_query: str

class FlightQueryResponse(pydantic.BaseModel):
    query_response: str
    