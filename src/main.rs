use actix_web::{web, App, HttpServer};
use actix_files as fs;

mod controllers;
mod models;
mod views;

use controllers::counter_controller::{add_one, show_count};
use controllers::item_controller::{add_item, delete_item, get_items, update_item};
use models::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/api/count", web::to(show_count))
            .route("/api/add", web::to(add_one))
            .route("/api/items", web::get().to(get_items)) // Fetch all items
            .route("/api/items", web::post().to(add_item)) // Add new item
            .route("/api/items/{id}", web::put().to(update_item)) // Update item
            .route("/api/items/{id}", web::delete().to(delete_item)) // Delete item
            .service(fs::Files::new("/", "./static").index_file("index.html")) // Serve static files
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
