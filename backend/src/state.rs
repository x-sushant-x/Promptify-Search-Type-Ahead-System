use std::sync::Arc;

use redis::Connection;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Arc<PgPool>,
    pub redis_conn: Arc<Connection>
}

impl AppState {
    pub fn new(pg_pool: PgPool, redis_conn: Connection) -> Self {
        AppState {
            pg_pool: Arc::new(pg_pool),
            redis_conn: Arc::new(redis_conn)
        }
    }
}
