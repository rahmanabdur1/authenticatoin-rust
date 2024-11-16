use actix_web::{web, HttpResponse};
use crate::services::todo_service::TodoService;
use crate::models::todo::Todo;

pub async fn get_all_todos(service: web::Data<TodoService>) -> HttpResponse {
    match service.get_all_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_todo(
    service: web::Data<TodoService>, 
    todo: web::Json<Todo>
) -> HttpResponse {
    match service.create_todo(&todo.title).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_todo(
    service: web::Data<TodoService>, 
    todo: web::Json<Todo>
) -> HttpResponse {
    match service.update_todo(&todo.id, todo.completed).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_todo(
    service: web::Data<TodoService>, 
    todo: web::Json<Todo>
) -> HttpResponse {
    match service.delete_todo(&todo.id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
