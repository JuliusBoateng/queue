use actix_web::{get, post, web, HttpResponse, Responder};
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
    println!("result {:?}", result);
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

#[get("/insert")]
pub async fn queue_insert() -> impl Responder {
    // pub id: String,
    // pub course: String,
    // pub name: String,
    // pub start: DateTime<Utc>,
    // pub end: DateTime<Utc>,
    // pub location: String,
    // pub students: Vec<Student>,
    // let ta = TA {id: String::from("Test"), course: String::from("Test"), name: String::from("Test"), start: chrono::offset::Utc::now(), end: chrono::offset::Utc::now(), location: String::from("Test"), students: vec![]};
    format!("foobar")
    // format!("Done")
    // db::insert_ta(db, ta);
}