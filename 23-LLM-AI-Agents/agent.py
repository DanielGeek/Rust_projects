import asyncio

from langchain_google_genai import ChatGoogleGenerativeAI
from browser_use import Agent
from pydantic import SecretStr
from dotenv import load_dotenv
import os

load_dotenv()

api_key = os.getenv("GOOGLE_API_KEY")

llm = ChatGoogleGenerativeAI(model="gemini-2.0-flash-exp", api_key=SecretStr(api_key))

async def run():
    agent = Agent(
        task=("Search for economy flights from Singapore to Perth and get top 5"),
        llm=llm,
    )

    await agent.run(max_steps=100)

if __name__ == "__main__":
    asyncio.run(run())