use crate::models::todo::Todo;
use async_trait::async_trait;
use mongodb::{Client, Database};
use mongodb::bson::{doc, Bson};
use futures::stream::StreamExt;
use futures::TryStreamExt;

pub struct TodoService {
    db: Database,
}

impl TodoService {
    pub fn new() -> Self {
        let client = Client::with_uri_str("mongodb+srv://actix:dZTp62o6udopYjdT@cluster0.gzkpw83.mongodb.net/todo_app?retryWrites=true&w=majority&appName=Cluster0").unwrap();
        let db = client.database("todo_db");
        TodoService { db }
    }

    pub async fn get_all_todos(&self) -> Vec<Todo> {
        let collection = self.db.collection("todos");
        let mut cursor = collection.find(None, None).await.unwrap();
        let mut todos = Vec::new();

        while let Some(doc) = cursor.try_next().await.unwrap() {
            let todo: Todo = bson::from_bson(Bson::Document(doc)).unwrap();
            todos.push(todo);
        }

        todos
    }

    pub async fn create_todo(&self, todo: Todo) -> Todo {
        let collection = self.db.collection("todos");
        let todo_doc = bson::to_document(&todo).unwrap();
        collection.insert_one(todo_doc, None).await.unwrap();
        todo
    }

    pub async fn update_todo(&self, todo: Todo) -> Option<Todo> {
        let collection = self.db.collection("todos");
        let filter = doc! {"_id": &todo.id};
        let update = doc! {
            "$set": {
                "title": &todo.title,
                "completed": &todo.completed,
            }
        };
        collection.update_one(filter, update, None).await.unwrap();

        Some(todo)
    }

    pub async fn delete_todo(&self, id: String) -> bool {
        let collection = self.db.collection("todos");
        let filter = doc! {"_id": &id};
        let result = collection.delete_one(filter, None).await.unwrap();
        result.deleted_count > 0
    }
}

