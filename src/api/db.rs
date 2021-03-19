use std::env;
use mongodb::{Client, options::{ClientOptions, FindOptions}, bson::{doc, Bson}};
//use futures::stream::StreamExt; 
 

// Put database functions in this file 
pub fn test_hello() -> String{ 
    println!("This is the test func in db.rs!"); 
    return "String from test_hello() in api/db!".to_string()
} 

pub async fn set_up_db() -> mongodb::Client{
    // Parse a connection string into an options struct.
    let db_connect = env::var("DBCONNECT").expect("Missing DBCONNECT environment variable");
    let mut client_options = ClientOptions::parse(&db_connect).await.expect("Can't Connect to Mongo");
    
    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Can't get a handle to deployment");
    
    return client
    //return "I set up the database connection!".to_string() 
}

pub async fn set_up_collection(client: mongodb::Client) -> String { 
    // List the names of the databases in that deployment.
    println!("These are our databases:"); 
    for db_name in client.list_database_names(None, None).await.expect("can't print database name") {
        println!("{}", db_name);
    }   
    let db = client.database("ourdb");
    println!("These are our collections:"); 
    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await.expect("can't print collection name") {
        println!("{}", collection_name);
    }   
    /*
    let collection = db.collection("queues"); 
    let docs = vec![ 
        doc! {"course": "20289", "name": "systems"}, 
        doc! {"course": "40842", "name": "hackers"}, 
        doc! {"course": "40822", "name": "cloud"},
    ]; 
    collection.insert_many(docs, None).await.expect("can't insert docs"); 
    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "name": "cloud" };
    let find_options = FindOptions::builder().sort(doc! { "course": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await.expect("no cursor");
    
    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(course) = document.get("course").and_then(Bson::as_str) {
                    println!("course: {}", course);
                }  else {
                    println!("no course found");
                }
            }
        }
    }   */
    return "string from set_up_collection".to_string() 

} 

