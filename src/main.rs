use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use actix_web::rt::System;
use futures::future::BoxFuture;
use crate::services::todo_service::TodoService;
use crate::models::todo::Todo;

mod services;
mod models;
mod config;

async fn get_all_todos(todo_service: web::Data<TodoService>) -> HttpResponse {
    let todos = todo_service.get_all_todos().await;
    HttpResponse::Ok().json(todos)
}

async fn create_todo(todo_service: web::Data<TodoService>, item: web::Json<Todo>) -> HttpResponse {
    let todo = todo_service.create_todo(item.into_inner()).await;
    HttpResponse::Created().json(todo)
}

async fn update_todo(todo_service: web::Data<TodoService>, item: web::Json<Todo>) -> HttpResponse {
    let updated = todo_service.update_todo(item.into_inner()).await;
    if updated.is_some() {
        HttpResponse::Ok().json(updated)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(todo_service: web::Data<TodoService>, id: web::Path<String>) -> HttpResponse {
    let result = todo_service.delete_todo(id.into_inner()).await;
    if result {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    actix_web::rt::System::new().block_on(async {
        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default()) // Enable logging
                .app_data(web::Data::new(TodoService::new())) // Shared data
                .route("/todos", web::get().to(get_all_todos))
                .route("/todos", web::post().to(create_todo))
                .route("/todos", web::put().to(update_todo))
                .route("/todos/{id}", web::delete().to(delete_todo))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    })
}
