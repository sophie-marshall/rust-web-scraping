use actix_web::{get, web, post, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;
use actix_web::http::header;

mod api;
mod models;
mod repository;

// define response struct to respond to client
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

// define response struct to respond to client
#[derive(Serialize, Deserialize)]
pub struct Input {
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

#[post("/uppercase")]
async fn uppercase(input: web::Json<Input>) -> impl Responder {
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

#[actix_web::main] // use actix_web::main to run as an async operation using actix-web runtime
async fn main() -> std::io::Result<()> {
    let url_db = repository::database::Database::new();
    let app_data = web::Data::new(url_db);

    
    // create a new server using HttpServer struct
    HttpServer::new(move || {
        // define cors policy
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .service(uppercase) // add new uppercase endpoint -- SM move this to API
            .default_service(web::route().to(not_found)) // app is used to register routes the server should handle, set default handler as not_found used if resource is not registered with teh server
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
        })
        .bind(("127.0.0.1", 8080))? // server listens on localhost:8080 and start it
        .run()
        .await
}
