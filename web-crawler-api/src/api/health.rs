use actix_web::{get, Responder};
use actix_web::HttpResponse;
use crate::models::response::Response;

// HEALTHCHECK ENDPOINT //
#[get("/health")]
pub async fn healthcheck () -> impl Responder {
    let response = Response {
        message: "Health check passed".to_string(),
    };
    HttpResponse::Ok().json(response)
}
