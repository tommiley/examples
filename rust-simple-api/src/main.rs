use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://0.0.0.0:8080");
    
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
} 