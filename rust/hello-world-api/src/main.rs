use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/hello/{name}")]
async fn hello_name(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let response = HelloResponse {
        message: format!("Hello, {}!", name),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port from environment variable or use 8080 as default
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    println!("Starting server at http://{}", bind_address);
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_name)
    })
    .bind(&bind_address)?
    .run()
    .await
} 