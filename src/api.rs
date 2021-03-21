use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;
use queue::TA;

mod db; 



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
    let mystring = db::test_hello();
    let db = db::connect_to_db().await;
    format!("{:?}", db)
    // db::insert_ta(db, )
    // let dbstring = db::set_up_collection(dbclient).await; 
    // format!("Queue {}! mystring: {} dbstring: {}", qid, mystring, dbstring) 
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
    let ta = TA {id: String::from("Test"), course: String::from("Test"), name: String::from("Test"), start: chrono::offset::Utc::now(), end: chrono::offset::Utc::now(), location: String::from("Test"), students: vec![]};
    format!("{:?}", ta)
    // format!("Done")
    // db::insert_ta(db, ta);
}

#[get("/insert_test")]
pub async fn queue_insert_test() -> impl Responder {
    // pub id: String,
    // pub course: String,
    // pub name: String,
    // pub start: DateTime<Utc>,
    // pub end: DateTime<Utc>,
    // pub location: String,
    // pub students: Vec<Student>,
    // let ta = TA {id: String::from("Test"), course: String::from("Test"), name: String::from("Test"), start: chrono::offset::Utc::now(), end: chrono::offset::Utc::now(), location: String::from("Test"), students: vec![]};
    // format!("{:?}", ta)
    let db = db::connect_to_db().await;
    db::insert_test(db).await;
    format!("Done")
    // format!("Done")
    // db::insert_ta(db, ta);
}