use axum::Router;
use tokio;
use std::sync::Arc;
use dotenvy;

use sqlx::postgres::PgPoolOptions;

mod routes;
mod utils;
mod config;
mod types;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool
}

#[tokio::main]
async fn main() {
    // Initialize env variables
    dotenvy::dotenv().ok();

    let db_url = &config::get_config()["DATABASE_URL"];
    let port = &config::get_config()["PORT"];

    println!("Connecting to db...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("Couldn't connect to the database");
    println!("Connected to db");

    let state = Arc::new(AppState {
        db_pool: pool
    });

    // Set up the Axum app
    let app = Router::new()
        .nest("/account", routes::account::get_routes(&state))
        .nest("/task", routes::task::get_routes(&state))
        .nest("/calendar", routes::calendar::get_routes(&state));

    let addr = format!("0.0.0.0:{}", port);
    println!("Rical backend v{} is listening on {}", option_env!("CARGO_PKG_VERSION").unwrap_or("?"), addr);
    
    // Run with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
