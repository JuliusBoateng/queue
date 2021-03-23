use actix_web::{get, post, web, HttpResponse, Responder};

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
