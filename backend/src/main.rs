mod config;
mod api;
mod models;
mod services;
mod database;
mod state;
mod repository;

use api::server;
use config::envconfig::Config;
use dotenv::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::load();

    let pg_pool = database::pg_connect::connect().await.expect("unable to connect to database");

    let redis = database::redis::connect();
    let redis_conn;

    match redis {
        Ok(conn) => redis_conn = conn,
        Err(err) => panic!("unable to connect to redis: {}", err)
    }

    let app_state = state::AppState::new(pg_pool, redis_conn);

    server::start_server(config, app_state).await
}