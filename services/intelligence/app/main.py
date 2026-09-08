from fastapi import FastAPI

from app.config import get_settings

app = FastAPI(title="groundwork-intelligence")


@app.get("/health")
def health():
    settings = get_settings()
    return {
        "status": "ok",
        "service": "groundwork-intelligence",
        "groq_model": settings.groq_model,
        "embedding_model": settings.embedding_model,
        "groq_key_configured": bool(settings.groq_api_key),
    }
