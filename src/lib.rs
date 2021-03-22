use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TA {
    #[serde(rename(deserialize = "_id"))]
    pub id: ObjectId,
    pub course: String,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub location: String,
    pub students: Vec<Student>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub time: DateTime<Utc>,
    pub desc: String,
}
