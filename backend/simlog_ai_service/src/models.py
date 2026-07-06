from uuid import UUID
import pydantic

class FlightQueryPayload(pydantic.BaseModel):
    flight_id: UUID
    user_query: str

class FlightQueryResponse(pydantic.BaseModel):
    query_response: str
    