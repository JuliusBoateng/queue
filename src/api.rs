use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/{name}")]
pub async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}
