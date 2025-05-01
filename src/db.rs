use sqlx::postgres::{PgPoolOptions,PgPool};
use std::env;
use dotenv::dotenv;

pub async fn connect() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
