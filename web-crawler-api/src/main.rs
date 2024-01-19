use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_cors::Cors;
use actix_web::http::header;

mod api;
mod models;
use models::response::Response;
mod repository;

// set default response service
async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string()
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main] // use actix_web::main to run as an async operation using actix-web runtime
async fn main() -> std::io::Result<()> {

    // lol have no idea what this is doing -- maybe revisit later 
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
            .service(api::api::healthcheck)
            .service(api::api::uppercase) // add new uppercase endpoint
            .service(api::crawl::crawl_url) // add base crawl endpoint
            .default_service(web::route().to(not_found)) // app is used to register routes the server should handle, set default handler as not_found used if resource is not registered with teh server
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
        })
        .bind(("127.0.0.1", 8080))? // server listens on localhost:8080 and start it
        .run()
        .await
}
