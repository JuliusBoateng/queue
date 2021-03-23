use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
    q: String,
}

#[get("/queues")]
pub async fn queue_all(
    app_data: web::Data<crate::AppState>,
) -> impl Responder {
    let action = app_data.service_container.user.get_all().await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/queues")]
pub async fn queue_create(
    app_data: web::Data<crate::AppState>,
    new_ta: web::Json<queue::TA>,
) -> impl Responder {
    let action = app_data.service_container.user.create_ta(&new_ta).await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/queues/{qid}")]
pub async fn queue_get(
    app_data: web::Data<crate::AppState>,
    web::Path(qid): web::Path<String>,
) -> impl Responder { 
    let action = app_data.service_container.user.get_by_id(&qid).await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => {
            match result {
                Some(result) => HttpResponse::Ok().json(result),
                None => HttpResponse::NotFound().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// // This handler gets called only if the request's query string contains `id` and `response_type` fields.
// // The correct request for this handler would be `/index.html?id=64&response_type=Code"`.
// async fn index(web::Query(info): web::Query<AuthRequest>) -> String {
//     format!("Authorization request for client with id={} and type={:?}!", info.id, info.response_type)
// }

#[get("/queues/search")]
pub async fn queue_search(
    app_data: web::Data<crate::AppState>,
    web::Query(search): web::Query<Search>,
) -> impl Responder {
    format!("{:?}", search.q);
     HttpResponse::Ok().json(search.q)
}
// pub async fn queue_search(
//     app_data: web::Data<crate::AppState>,
//     web::Query(info): web::Query<String>,
// ) -> impl Responder {
//     format!("{:?}", info)
// }

#[delete("/queues/{qid}")]
pub async fn queue_delete(
    app_data: web::Data<crate::AppState>,
    web::Path(qid): web::Path<String>,
) -> impl Responder { 
    let action = app_data.service_container.user.delete_by_id(&qid).await;
    let result = web::block(|| action).await;
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// FOR TESTING ONLY
#[get("/insert/{name}")]
pub async fn insert_test(
    app_data: web::Data<crate::AppState>,
    web::Path(name): web::Path<String>
) -> impl Responder {
    let action = app_data.service_container.user.create(&name).await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
