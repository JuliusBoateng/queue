use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(queue::Student {
        id: "abc1234".to_string(),
        name: "John Smith".to_string(),
        time: Utc::now(),
        desc: "Help with queues".to_string(),
    })
}

#[get("/{name}")]
pub async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}
