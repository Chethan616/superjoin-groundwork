use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub intelligence_url: String,
}

impl AppState {
    pub async fn connect() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://groundwork:groundwork@localhost:5433/groundwork".into());
        let intelligence_url = std::env::var("INTELLIGENCE_URL")
            .unwrap_or_else(|_| "http://localhost:8000".into());

        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;

        Ok(Self { db, intelligence_url })
    }
}
