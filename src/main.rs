use actix_web::{App, HttpServer};
use std::env;
use mongodb::{Client, options::ClientOptions};

mod api; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse a connection string into an options struct.
    let db_connect = env::var("DBCONNECT").expect("Missing DBCONNECT environment variable");
    let mut client_options = ClientOptions::parse(&db_connect).await.expect("Can't Connect to Mongo");
    
    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Can't get a handle to deployment");

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await.expect("Can't list database names") {
        println!("It worked: {}", db_name);
    }

    let port:u32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(api::queue_all)
            .service(api::queue_get)
            .service(api::create_queue)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

