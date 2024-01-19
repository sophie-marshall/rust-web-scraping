use actix_web::{web, get, Responder};
use actix_web::{web::{Data,Json,}, post, HttpResponse};
use crate::{models::url::Url, 
    models::response::Response, 
    models::input::Input, 
    repository::database::Database};

// HEALTHCHECK ENDPOINT //
#[get("/health")]
pub async fn healthcheck () -> impl Responder {
    let response = Response {
        message: "Health check passed".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// UPPERCASE ENDPOINT //
#[post("/uppercase")]
pub async fn uppercase(input: web::Json<Input>) -> impl Responder {
    let uppercase_data = input.message.to_uppercase();
    let response = Response {
        message: uppercase_data,
    };

    // Serialize Response to JSON string
    match serde_json::to_string(&response) {
        Ok(json_response) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json_response),
        Err(e) => {
            eprintln!("Failed to serialize response to JSON: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}













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