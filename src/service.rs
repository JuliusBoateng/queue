use std::env;
use mongodb::{Client, options::ClientOptions, error::Error, Collection, bson::{doc, oid::ObjectId, from_bson, Bson, ser::to_document}};
use futures_lite::stream::StreamExt; 

#[derive(Clone)]
pub struct QueueService {
    ta_collection: Collection,
    student_collection: Collection, 
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

impl QueueService {
    pub fn new(ta_collection: Collection, student_collection: Collection) -> QueueService {
        QueueService { ta_collection, student_collection }
    }

    pub async fn convert_ta_reponse(&self, ta: queue::TA) -> queue::TAResponse {
        let mut new_students =  Vec::<queue::Student>::new();
        
        for student_id in &ta.students {
            let obj = QueueService::student_get_by_id(&self, &student_id).await.expect("Get students failed");
            new_students.push(obj.unwrap())
        }
        
       let response = queue::TAResponse {
            id: ta.id,
            course: ta.course,
            name: ta.name,
            start: ta.start,
            end: ta.end,
            location: ta.location,
            students: new_students,
        };

        return response
    }

    pub async fn create_ta(&self, new_ta: &queue::TA) -> Result<String, Error> {
        let new_ta_doc = to_document(new_ta).unwrap();
        let insert_result = self.ta_collection.insert_one(new_ta_doc, None).await?;
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap())
    }
    
    pub async fn create_student(&self, new_student: &queue::Student, qid: &str) -> Result<String, Error> {
        // add student to student collection 
        let new_student_doc = to_document(new_student).unwrap();
        let insert_result = self.student_collection.insert_one(new_student_doc, None).await?;
        
        // add student id to queue 
        let qoid = ObjectId::with_string(qid);
        /*// If the id is malformed, return None (404)
        if let Err(_) = qoid {
            return Ok(None);
        }*/
        let qfilter = doc! {"_id": qoid.unwrap()};
        let cursor = self.ta_collection.find_one(doc! {"_id": ObjectId::with_string(qid).unwrap()}, None).await?; // return just students from ta queue 
        match cursor{ 
            None => println!("Error: create_student did not find a queue with entered qid"),  
            Some(doc) => if true {  
                    let ta_struct = from_bson::<queue::TA>(Bson::Document(doc)).unwrap();
                    let student_vector = ta_struct.students; 
                    let svlen = student_vector.len();
                    let sid = insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap();  
                    let soid = sid.to_string(); 
                    let key = format!("{}{}", "students.", svlen.to_string()); 
                    let student_update = doc! {"$set": {key: soid}};
                    let _effect = self.ta_collection.update_one(qfilter, student_update,  None ).await?; 
                },  
        }          
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap()) 
    }


    pub async fn update_ta(&self, updates: &queue::TA, id: &str) -> Result<Option<queue::TAResponse>, Error> {
        let oid = ObjectId::with_string(id); 
        if let Err(_) = oid { 
            return Ok(None); 
        } 
       
        let update_doc = doc! {"$set": to_document(updates).unwrap()};
        let _effect = self.ta_collection.update_one(doc! {"_id": ObjectId::with_string(id).unwrap()}, update_doc, None).await?;

        let res = self.ta_collection.find_one(doc! {"_id": ObjectId::with_string(id).unwrap()}, None).await?; // changed to match get_by_id
        match res{ // used to have .unwrap()  
            None => Ok(None),
            Some(doc) => {
                let ta_struc = from_bson(Bson::Document(doc)).unwrap();
                let new_struc = QueueService::convert_ta_reponse(&self,ta_struc).await;
                let new_doc =  to_document(&new_struc).unwrap();
                Ok(from_bson(Bson::Document(new_doc)).unwrap())
            }
        }
    }
 
    pub async fn update_student(&self, updates: &queue::Student, sid: &str) -> Result<Option<queue::Student>, Error> {
        let soid = ObjectId::with_string(sid); 
        if let Err(_) = soid { 
            return Ok(None); 
        } 

        let update_doc = doc! {"$set": to_document(updates).unwrap()};
        let _effect = self.student_collection.update_one(doc! {"_id": ObjectId::with_string(sid).unwrap()}, update_doc, None).await?;

        let res = self.student_collection.find_one(doc! {"_id": ObjectId::with_string(sid).unwrap()}, None).await?; // changed to match get_by_id
        match res{ // used to have .unwrap()  
            None => {
                println!("Nooo");
                Ok(None)
            },
            Some(doc) => Ok(from_bson(Bson::Document(doc)).unwrap())
        }
    }


    pub async fn get_all(&self) -> Result<Vec<queue::TAResponse>, Error> {
        let mut cursor = self.ta_collection.find(None, None).await?;
        // let result = cursor.collect::<queue::TA>().await?;
        let mut result = Vec::<queue::TAResponse>::new();
        while let Some(item) = cursor.next().await {
            if let Ok(doc) = item {
                let ta_struc = from_bson(Bson::Document(doc)).unwrap();
                let new_struc = QueueService::convert_ta_reponse(&self,ta_struc).await;
                let new_doc =  to_document(&new_struc).unwrap();
                result.push(from_bson(Bson::Document(new_doc)).unwrap());
            }
        }
        Ok(result)
    }

    pub async fn ta_get_by_id(&self, id: &str) -> Result<Option<queue::TAResponse>, Error> {
        let oid = ObjectId::with_string(id);
        // If the id is malformed, return None (404)
        if let Err(_) = oid {
            return Ok(None);
        }
        let filter = doc! {"_id": oid.unwrap()};
        let result = self.ta_collection.find_one(filter, None).await?;
        match result {
            None => Ok(None),
            Some(doc) => {
                let ta_struc = from_bson(Bson::Document(doc)).unwrap();
                let new_struc = QueueService::convert_ta_reponse(&self,ta_struc).await;
                Ok(Some(new_struc))
            }
        }
    }

    pub async fn student_get_by_id(&self, sid: &str) -> Result<Option<queue::Student>, Error> {
        let soid = ObjectId::with_string(sid);
        // If the id is malformed, return None (404)
        if let Err(_) = soid {
            return Ok(None);
        }
        let filter = doc! {"_id": soid.unwrap()};
        let result = self.student_collection.find_one(filter, None).await?;
        match result {
            None => Ok(None),
            Some(doc) => Ok(from_bson(Bson::Document(doc)).unwrap()),
        }
    }

    pub async fn student_get_all(&self) -> Result<Vec<queue::Student>, Error> {
        let mut cursor = self.student_collection.find(None, None).await?;
        let mut result = Vec::<queue::Student>::new();
        while let Some(item) = cursor.next().await {
            if let Ok(doc) = item {
                result.push(from_bson(Bson::Document(doc)).unwrap());
            }
        }
        Ok(result)
    }

    pub async fn queue_delete_by_id(&self, id: &str) -> Result<Option<()>, Error> {
        let oid = ObjectId::with_string(id);
        // If the id is malformed, return None (404)
        if let Err(_) = oid {
            return Ok(None);
        }
        let filter = doc! {"_id": oid.unwrap()};
        self.ta_collection.delete_one(filter, None).await?;
        Ok(Some(()))
    }

    pub async fn student_delete_by_id(&self, qid: &str, sid: &str) -> Result<Option<()>, Error> {
        // If the id is malformed, return None (404)
        if let Err(_) = ObjectId::with_string(qid) {
            return Ok(None);
        }

        if let Err(_) = ObjectId::with_string(sid) {
            return Ok(None);
        }
        
        let qfilter = doc! {"_id": ObjectId::with_string(qid).unwrap()};
        let qresult = self.ta_collection.find_one(qfilter, None).await?;

        match qresult {
            None => println!("Error: student_delete did not find a queue with entered qid"),  
            Some(doc) => if true {
                let ta_struct = from_bson::<queue::TA>(Bson::Document(doc)).unwrap();
                let mut student_vector = ta_struct.students;
                let mut found = false;
                let mut index: usize = usize::MAX;
                
                for (pos, student_id) in student_vector.iter().enumerate() {
                    if student_id.eq(sid) {
                        found = true;
                        index = pos;
                        break;
                    }
                }

                if found == true {
                    student_vector.remove(index);
                    let student_update = doc! {"$set": {"students": student_vector}};
                    let qfilter = doc! {"_id": ObjectId::with_string(qid).unwrap()};
                    self.ta_collection.update_one(qfilter, student_update, None).await?;
                } else {
                    return Ok(None);
                }
            },  
        }

        let sfilter = doc! {"_id": ObjectId::with_string(sid).unwrap()};
        let  _student_effect = self.student_collection.delete_one(sfilter, None).await?;
        
        if _student_effect.deleted_count == 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }

    // FOR TESTING ONLY
    pub async fn create(&self, name: &str) -> Result<String, Error> {
        let new_ta = queue::TA {
            id: None,
            course: "Course".to_string(),
            name: name.to_string(),
            start: chrono::Utc::now(),
            end: chrono::Utc::now(),
            location: "Location".to_string(),
            students: Vec::new(),
        };
        let new_ta_doc = to_document(&new_ta).unwrap();
        let insert_result = self.ta_collection.insert_one(new_ta_doc, None).await?;
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap())
    }
}
