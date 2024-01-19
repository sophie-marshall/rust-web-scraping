use actix_web::web;
use actix_web::{web::{Data,Json,}, post, HttpResponse};
use crate::{models::url::Url, repository::database::Database};

// create a create_url function taking DB and Url as input
#[post("/urls")]
pub async fn create_url(db: Data<Database>, new_url: Json<Url>) -> HttpResponse {
    // call function and return the result to the caller
    let url = db.create_url(new_url.into_inner());
    // handle success/failure cases
    match url {
        Ok(url) => HttpResponse::Ok().json(url),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// create a config function to reguster API endpoints
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_url)
    );
}