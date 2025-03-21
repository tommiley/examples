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
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_name)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 