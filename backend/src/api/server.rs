use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::api::{auto_complete_handler, auto_complete_handler_v2};
use crate::config::envconfig::Config;
use crate::services;
use crate::state::AppState;

pub async fn start_server(config: Config, state: AppState) -> std::io::Result<()> {
    println!("🚀 Server started successfully");

    let svc = services::trie_query::TrieQuerySVC::new();

    let handler = auto_complete_handler_v2::AutoCompleteHandlerV2 {
        svc: Arc::new(Mutex::new(svc)),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(setup_cors())
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(handler.clone()))
            .configure(|cfg| auto_complete_handler::init_routes(cfg, state.pg_pool.clone()))
            .configure(|cfg| auto_complete_handler_v2::init_routes(cfg))
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}

fn setup_cors() -> Cors {
    let cors = Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin();

    cors
}
