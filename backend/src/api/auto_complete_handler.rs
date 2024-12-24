use std::sync::Arc;

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{models::response_models, repository, services};

struct AutoCompleteHandler {
    svc: services::query::QuerySVC,
}

#[post("/query/{query}")]
async fn add_query(
    query: web::Path<String>,
    handler: web::Data<AutoCompleteHandler>,
) -> impl Responder {
    let query_string = query.into_inner();

    let resp = handler.svc.add_new_query(query_string).await;

    match resp {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            let error = response_models::Error::new(format!("{}", err));
            HttpResponse::InternalServerError().body(error)
        }
    }
}

#[get("/suggestions/{query}")]
async fn get_suggestions(
    query: web::Path<String>,
    handler: web::Data<AutoCompleteHandler>,
) -> impl Responder {
    let query_string = query.into_inner();

    let resp = handler.svc.get_suggestions(query_string).await;

    let response = response_models::Success::new(resp);

    HttpResponse::Ok().body(response)
}

pub fn init_routes(cfg: &mut web::ServiceConfig, db: Arc<PgPool>) {
    let repo = repository::query::QueryRepo::new(db);
    let svc = services::query::QuerySVC::new(repo);

    let handler = AutoCompleteHandler { svc };

    cfg.app_data(web::Data::new(handler));

    cfg.service(
        web::scope("/api/v1")
            .service(add_query)
            .service(get_suggestions),
    );
}
