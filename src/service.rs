use mongodb::{error::Error, Collection, bson::{doc, oid::ObjectId, from_bson, Bson}};

#[derive(Clone)]
pub struct QueueService {
    collection: Collection,
}

impl QueueService {
    pub fn new(collection: Collection) -> QueueService {
        QueueService { collection }
    }

    pub async fn create(&self, name: &str) -> Result<String, Error> {
        let insert_result = self.collection.insert_one(doc! {"name": name}, None).await?;
        Ok(insert_result.inserted_id.as_object_id().map(ObjectId::to_hex).unwrap())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<queue::TA>, Error> {
        let oid = ObjectId::with_string(id);
        if oid.is_err() {
            return Ok(None);
        }
        let filter = doc! {"_id": oid.unwrap()};
        println!("id {:?}", filter);
        let result = self.collection.find_one(filter, None).await?;
        println!("result {:?}", result);
        match result {
            None => Ok(None),
            Some(doc) => Ok(from_bson(Bson::Document(doc)).unwrap()),
        }
    }
}
