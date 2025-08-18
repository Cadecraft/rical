use axum::Router;
use tokio;
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use dotenv;

use sqlx::postgres::PgPoolOptions;

mod routes;

const PORT: &str = "3001";

struct AppStateData {
    // TODO: put db here and process users correctly
    temporary_testing_users: HashMap<String, String>
}

#[derive(Clone)]
pub struct AppState {
    data: Arc<tokio::sync::Mutex<AppStateData>>
}

#[tokio::main]
async fn main() {
    // Initialize env variables
    dotenv::dotenv().ok();

    let db_url = env::var("DB_URL").expect("DB_URL must be set");

    /*let state = AppState {
        data: Arc::new(tokio::sync::Mutex::new(AppStateData {
            temporary_testing_users: HashMap::new()
        }))
    };*/

    println!("Connecting to db...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to the database");
    println!("Connected to db");

    let app = Router::new()
        .merge(routes::user::get_routes(/*&state*/));

    let addr = format!("0.0.0.0:{}", PORT);
    println!("Rical backend v{} is listening on {}", option_env!("CARGO_PKG_VERSION").unwrap_or("?"), addr);
    
    // Run with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
