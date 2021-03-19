use std::env;
use mongodb::{Client, options::ClientOptions};

// Put database functions in this file 
pub fn test_hello() -> String{ 
    println!("This is the test func in db.rs!"); 
    return "String from test_hello() in api/db!".to_string()
} 

pub async fn set_up_db() -> String{
    // Parse a connection string into an options struct.
    let db_connect = env::var("DBCONNECT").expect("Missing DBCONNECT environment variable");
    let mut client_options = ClientOptions::parse(&db_connect).await.expect("Can't Connect to Mongo");
    
    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Can't get a handle to deployment");

    return "I set up the database connection!".to_string() 
}

