use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

//cockroach db connection
pub async fn connect_cockroach() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database_URL fetching failed");
    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to db")
}
 