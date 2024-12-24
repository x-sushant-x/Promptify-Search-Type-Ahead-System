use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Arc<PgPool>,
}

impl AppState {
    pub fn new(pg_pool: PgPool) -> Self {
        AppState {
            pg_pool: Arc::new(pg_pool),
        }
    }
}
