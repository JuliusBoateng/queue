use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
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
 
#[post("/queues/{qid}/students")]
pub async fn student_create(
    app_data: web::Data<crate::AppState>,
    web::Path(qid): web::Path<String>, 
    new_student: web::Json<queue::Student>,
) -> impl Responder {
    let action = app_data.service_container.user.create_student(&new_student, &qid).await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[put("/students/{sid}")]
pub async fn student_update( 
    app_data: web::Data<crate::AppState>, 
    web::Path(sid): web::Path<String>, 
    updates: web::Json<queue::Student>,
) -> impl Responder { 
    let action = app_data.service_container.user.update_student(&updates, &sid).await;
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

#[get("/queues/{qid}/students/{sid}")]
pub async fn student_delete(
    app_data: web::Data<crate::AppState>,
    web::Path(qid): web::Path<String>, 
    web::Path(sid): web::Path<String>, 
) -> impl Responder { 
    let action = app_data.service_container.user.student_delete_by_id(&qid, &sid).await;
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

#[get("/queues/search")]
pub async fn queue_search(
    app_data: web::Data<crate::AppState>,
    web::Query(search): web::Query<Search>,
) -> impl Responder {
    let action = app_data.service_container.user.get_all().await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => {
            let mut query_rs = Vec::<queue::TA>::new();

            for b in result {
                if b.course.eq(&search.q) || b.name.eq(&search.q) || b.location.eq(&search.q) {
                    query_rs.push(b);
                }
            }

            HttpResponse::Ok().json(query_rs)
        }

        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/queues/{qid}")]
pub async fn queue_delete(
    app_data: web::Data<crate::AppState>,
    web::Path(qid): web::Path<String>,
) -> impl Responder { 
    let action = app_data.service_container.user.queue_delete_by_id(&qid).await;
    let result = web::block(|| action).await;
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
#[put("/queues/{qid}")] 
pub async fn queue_update( 
    app_data: web::Data<crate::AppState>, 
    web::Path(qid): web::Path<String>, 
    updates: web::Json<queue::TA>,
) -> impl Responder { 
    let action = app_data.service_container.user.update_ta(&updates, &qid).await;
    let result = web::block(|| action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
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
