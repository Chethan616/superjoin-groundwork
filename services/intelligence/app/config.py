import os
from functools import lru_cache

from dotenv import load_dotenv

load_dotenv()


class Settings:
    groq_api_key: str = os.getenv("GROQ_API_KEY", "")
    groq_model: str = os.getenv("GROQ_MODEL", "moonshotai/kimi-k2-instruct")
    groq_vision_model: str = os.getenv(
        "GROQ_VISION_MODEL", "meta-llama/llama-4-scout-17b-16e-instruct"
    )
    embedding_model: str = os.getenv("EMBEDDING_MODEL", "BAAI/bge-small-en-v1.5")
    port: int = int(os.getenv("INTELLIGENCE_PORT", "8000"))


@lru_cache
def get_settings() -> Settings:
    return Settings()
