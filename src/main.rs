use actix_web::{App, HttpServer};
use std::env;

mod api; 

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    let port:u32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(api::queue_all)
            .service(api::queue_get)
            .service(api::queue_insert)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

