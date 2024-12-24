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

    let app_state = state::AppState::new(pg_pool);

    server::start_server(config, app_state).await
}