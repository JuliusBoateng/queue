use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TA {
    #[serde(skip_serializing_if = "Option::is_none", rename(deserialize = "_id"))]
    pub id: Option<ObjectId>,
    pub course: String,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub location: String,
    pub students: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    #[serde(skip_serializing_if = "Option::is_none", rename(deserialize = "_id"))]
    pub id: Option<ObjectId>,
    pub name: String,
    pub time: DateTime<Utc>,
    pub desc: String,
}
