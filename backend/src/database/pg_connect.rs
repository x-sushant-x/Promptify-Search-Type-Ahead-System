use sqlx::PgPool;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let conn_string = "postgresql://postgres@localhost:5432/promptify";
        
    let pg_pool = PgPool::connect(&conn_string).await;

    pg_pool
}