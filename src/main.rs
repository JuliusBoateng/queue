use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::env;

mod api; 
mod service;

pub struct ServiceContainer {
  user: service::QueueService,
}

impl ServiceContainer {
  pub fn new(user: service::QueueService) -> Self {
    ServiceContainer { user }
  }
}

pub struct AppState {
  service_container: ServiceContainer,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let port:u32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    
    let database = service::connect_to_db().await;
    let ta_collection = database.collection("ta");
    let student_collection = database.collection("student"); 

    HttpServer::new(move || {
        let queue_service_worker = service::QueueService::new(ta_collection.clone(), student_collection.clone());
        let service_container = ServiceContainer::new(queue_service_worker);

        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .data(AppState { service_container })
            .service(api::queue_all)
            .service(api::student_create)
            .service(api::student_update) 
            .service(api::queue_create)
            .service(api::queue_search)
            .service(api::queue_get)
            .service(api::queue_delete)
            .service(api::queue_update)
            .service(api::insert_test)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

