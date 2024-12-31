use std::sync::{Arc, Mutex};

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::{models::response_models, services};

#[derive(Clone)]
pub struct AutoCompleteHandlerV2 {
    pub svc: Arc<Mutex<services::trie_query::TrieQuerySVC>>,
}

#[post("/query/{query}")]
async fn add_query(
    query: web::Path<String>,
    handler: web::Data<AutoCompleteHandlerV2>,
) -> impl Responder {
    let query_string = query.into_inner();

    let mut svc = handler.svc.lock().unwrap();

    svc.add_search_query(query_string);
    HttpResponse::Ok()
}

#[get("/suggestions/{query}")]
async fn get_suggestions(
    query: web::Path<String>,
    handler: web::Data<AutoCompleteHandlerV2>,
) -> impl Responder {
    let query_string = query.into_inner();
    let svc = handler.svc.lock().unwrap();

    let suggestions = svc.get_suggestions(query_string);

    let response = response_models::Success::new(suggestions);

    HttpResponse::Ok().body(response)
}

#[get("/insert-sample-data")]
async fn insert_sample_data(
    handler: web::Data<AutoCompleteHandlerV2>,
) -> impl Responder {

    handler.svc.lock().unwrap().insert_sample_data();

    let response = response_models::Success::new("Done");

    HttpResponse::Ok().body(response)
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .service(add_query)
            .service(get_suggestions)
            .service(insert_sample_data),
    );
}
