use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use serde::ser::SerializeStruct;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String, completed: bool) -> Self {
        Todo {
            id: ObjectId::new().to_string(),
            title,
            completed,
        }
    }
}
