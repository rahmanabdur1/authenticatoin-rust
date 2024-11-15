
use color_eyre::{Result, eyre::WrapErr};
use chrono::{Utc, Duration};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber;

#[actix_rt::main]
async fn main() -> Result<()> {
    // Set up logging using tracing-subscriber
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenv().ok();

    // Load configuration, handle potential errors with expect()
    let config = Config::from_env().expect("Server configuration");

    // Get current time
    let now = Utc::now();

    // Subtract 365 days from the current time
    let one_year_ago = now - Duration::days(365);

    // Output the result
    info!("One year ago: {}", one_year_ago);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
    })
    .bind(format!("{}:{}", config.host, config.port))? // Fixed typo and syntax errors here
    .run()
    .await?; // Start the server and await

    Ok(())
}
