use actix_web::{web, App, HttpServer};

use crate::api::auto_complete_handler;
use crate::config::envconfig::Config;
use crate::state::AppState;

pub async fn start_server(config: Config, state: AppState) -> std::io::Result<()> {
    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(|cfg| auto_complete_handler::init_routes(cfg, state.pg_pool.clone()))
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
