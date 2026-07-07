from models import FlightQueryPayload, FlightQueryResponse
from google import genai
from dotenv import load_dotenv
import os
import requests

# Load .env variables
load_dotenv()

# Get client API key
client = genai.Client(api_key=os.getenv("GEMINI_API_KEY" or ""))

# Get main backend address
backend_address = os.getenv("BACKEND_ADDRESS", "simlog_backend:3000")

# AI query handler
def query_ai(flight_query_payload: FlightQueryPayload):
    flight_data = requests.get(f"http://{backend_address}/flights/{flight_query_payload.flight_id}")
    flight_data = flight_data.json()
    
    response = client.models.generate_content(
        model="gemini-2.5-flash",
        contents=f"The user asked: {flight_query_payload.user_query} about this flight: {flight_data}"
    )

    return FlightQueryResponse(query_response=response.text or "No response from ai")
    