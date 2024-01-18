use actix_web::{get, web, post, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

mod api;
mod models;
mod repository;

// define response struct to respond to client
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

// define health handler to see if server is running
#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Health check passed".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string()
    };
    Ok(HttpResponse::NotFound().json(response))
}

// define response struct to respond to client
#[derive(Serialize, Deserialize)]
pub struct Input {
    pub message: String,
}

// 
#[post("/uppercase")]
async fn uppercase(input: web::Json<Input>) -> impl Responder {
    let uppercase_data = input.message.to_uppercase();
    let response = Response {
        message: uppercase_data,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main] // use actix_web::main to run as an async operation using actix-web runtime
async fn main() -> std::io::Result<()> {
    let url_db = repository::database::Database::new();
    let app_data = web::Data::new(url_db);
    
    // create a new server using HttpServer struct
    HttpServer::new(move || 
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .service(uppercase) // add new uppercase endpoint -- SM move this to API
            .default_service(web::route().to(not_found)) // app is used to register routes the server should handle, set default handler as not_found used if resource is not registered with teh server
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8080))? // server listens on localhost:8080 and start it
        .run()
        .await
}

