use axum::Router;
use tokio;
use std::sync::Arc;
use std::env;
use dotenv;

use sqlx::postgres::PgPoolOptions;

mod routes;
mod setup_schemas;
mod utils;

const PORT: &str = "3001";

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool
}

#[tokio::main]
async fn main() {
    // Initialize env variables
    dotenv::dotenv().ok();

    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    println!("Connecting to db...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to the database");
    println!("Connected to db");

    // Set up the database with schemas
    setup_schemas::setup_schemas(&pool).await;

    let state = Arc::new(AppState {
        db_pool: pool
    });

    // Set up the Axum app
    let app = Router::new()
        .nest("/account", routes::user::get_routes(&state));

    let addr = format!("0.0.0.0:{}", PORT);
    println!("Rical backend v{} is listening on {}", option_env!("CARGO_PKG_VERSION").unwrap_or("?"), addr);
    
    // Run with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
