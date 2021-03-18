use chrono::{NaiveDateTime};

pub struct Queue {
    pub id: String,
    pub course: String,
    pub name: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub location: String,
    pub students: Vec<Student>,
}

pub struct Student {
    pub id: String,
    pub name: String,
    pub desc: String,
}
