use actix_web::{web, Responder};
use crate::models::app_state::AppState;
use crate::views::response;

pub async fn show_count(data: web::Data<AppState>) -> impl Responder {
    let count = data.get_count();
    response::format_count(count)
}

pub async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let new_count = data.increment();
    response::format_count(new_count)
}
