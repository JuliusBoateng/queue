use std::env;
use mongodb::{Client, options::{ClientOptions, FindOptions}, bson::{doc, Bson}};
// use futures::stream::StreamExt;
 

// Put database functions in this file 
pub fn test_hello() -> String{ 
    println!("This is the test func in db.rs!"); 
    return "String from test_hello() in api/db!".to_string()
} 

pub async fn connect_to_db() -> mongodb::Database {
    // Parse a connection string into an options struct.
    let db_connect = env::var("DBCONNECT").expect("Missing DBCONNECT environment variable");
    let mut client_options = ClientOptions::parse(&db_connect).await.expect("Can't Connect to Mongo");
    
    // Manually set an option.
    client_options.app_name = Some("project queue".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Can't get a handle to deployment");

    let db = client.database("queue");

    return db
}

pub async fn insert_ta(db: mongodb::Database, ta: queue::TA) -> () {
    let collection = db.collection("ta");
    // let test = "\"title\": \"1984\", \"author\": \"George Orwell\"";
    // pub id: String,
    // pub course: String,
    // pub name: String,
    // pub start: DateTime<Utc>,
    // pub end: DateTime<Utc>,
    // pub location: String,
    // pub students: Vec<Student>,
    // format!(serde_json::to_string(&ta.Student).unwrap())
    let docs = vec![
        doc! { "uuid" : format!("{:?}", ta.id), "course" : format!("{}", ta.course), "name" : format!("{}", ta.name), "start" : format!("{}", ta.start), "end" : format!("{}", ta.end), "location" : format!("{}", ta.location), "students" : "[]"}
    ];
    collection.insert_many(docs, None).await.expect("Can't Insert into Mongo");
}

// pub async fn insert_ta_object()

// pub async fn set_up_collection(client: mongodb::Client) -> String { 


    // println!("These are our collections:"); 
    // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await.expect("can't print collection name") {
    //     println!("{}", collection_name);
    // }   
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
    // return "string from set_up_collection".to_string() 

// } 
