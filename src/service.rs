use mongodb::{error::Error, Collection, bson::{doc, oid::ObjectId, from_bson, Bson, ser::to_document}};
use futures_lite::stream::StreamExt;

#[derive(Clone)]
pub struct QueueService {
    collection: Collection,
}

impl QueueService {
    pub fn new(collection: Collection) -> QueueService {
        QueueService { collection }
    }

    pub async fn create_ta(&self, new_ta: &queue::TA) -> Result<String, Error> {
        let new_ta_doc = to_document(new_ta).unwrap();
        let insert_result = self.collection.insert_one(new_ta_doc, None).await?;
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap())
    }

    pub async fn get_all(&self) -> Result<Vec<queue::TA>, Error> {
        let mut cursor = self.collection.find(None, None).await?;
        // let result = cursor.collect::<queue::TA>().await?;
        let mut result = Vec::<queue::TA>::new();
        while let Some(item) = cursor.next().await {
            if let Ok(doc) = item {
                result.push(from_bson(Bson::Document(doc)).unwrap());
            }
        }
        Ok(result)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<queue::TA>, Error> {
        let oid = ObjectId::with_string(id);
        // If the id is malformed, return None (404)
        if let Err(_) = oid {
            return Ok(None);
        }
        let filter = doc! {"_id": oid.unwrap()};
        let result = self.collection.find_one(filter, None).await?;
        match result {
            None => Ok(None),
            Some(doc) => Ok(from_bson(Bson::Document(doc)).unwrap()),
        }
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<Option<()>, Error> {
        let oid = ObjectId::with_string(id);
        // If the id is malformed, return None (404)
        if let Err(_) = oid {
            return Ok(None);
        }
        let filter = doc! {"_id": oid.unwrap()};
        self.collection.delete_one(filter, None).await?;
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
        let insert_result = self.collection.insert_one(new_ta_doc, None).await?;
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap())
    }
}
