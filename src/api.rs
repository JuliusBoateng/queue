use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;

#[get("/queues")]
pub async fn queue_all() -> impl Responder {
    HttpResponse::Ok().json(queue::Student {
        id: "abc1234".to_string(),
        name: "John Smith".to_string(),
        time: Utc::now(),
        desc: "Help with queues".to_string(),
    })
}

#[get("/queues/{name}")]
pub async fn queue_get(web::Path(qid): web::Path<String>) -> impl Responder {
    format!("Queue {}!", qid)
}
