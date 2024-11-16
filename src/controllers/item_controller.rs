use actix_web::{web, HttpResponse, Responder};
use crate::models::{app_state::AppState, item::Item};

/// Fetch all items
pub async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.borrow();
    HttpResponse::Ok().json(&*items)
}

/// Add a new item
pub async fn add_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.borrow_mut();
    items.push(item.into_inner());
    HttpResponse::Created().finish()
}

/// Update an item by ID
pub async fn update_item(
    data: web::Data<AppState>,
    path: web::Path<usize>,
    item: web::Json<Item>,
) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.borrow_mut();
    if let Some(existing_item) = items.iter_mut().find(|i| i.id == id) {
        *existing_item = item.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

/// Delete an item by ID
pub async fn delete_item(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();
    let mut items = data.items.borrow_mut();
    if let Some(index) = items.iter().position(|i| i.id == id) {
        items.remove(index);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
