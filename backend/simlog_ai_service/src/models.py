from uuid import UUID
import pydantic

# User AI flight query payload model
class FlightQueryPayload(pydantic.BaseModel):
    flight_id: UUID
    user_query: str

# // User AI flight query response model
class FlightQueryResponse(pydantic.BaseModel):
    query_response: str
    