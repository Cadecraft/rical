use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    Json,
    Router
};

use crate::AppState;
use crate::utils;

use std::sync::Arc;

use serde::{Deserialize, Serialize};


use sqlx;

pub fn get_routes(state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .with_state(state.clone())
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCredentials>
) -> StatusCode {
    let matching_username: Option<String> = sqlx::query_scalar(
        "SELECT username FROM account WHERE username=$1;"
    ).bind(&payload.username)
        .fetch_optional(&state.db_pool)
        .await
        .expect("Could not select username");
    if matching_username.is_some() {
        // The username is already taken
        return StatusCode::CONFLICT;
    }
    let hashed_password = utils::hash_password(&payload.password);
    let res = sqlx::query("INSERT INTO account (username, hashed_password) VALUES ($1, $2)")
        .bind(&payload.username).bind(&hashed_password)
        .execute(&state.db_pool).await;
    if res.is_err() {
        // TODO: better error message
        return StatusCode::BAD_REQUEST;
    }
    println!("- User signed up with username: '{}'", payload.username);
    StatusCode::CREATED
}

#[derive(Serialize)]
struct AuthToken {
    token: String
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCredentials>
) -> (StatusCode, Json<AuthToken>) {
    // TODO: check against db, return response with auth token, etc.
    let auth_token = AuthToken {
        token: format!("log in example: {} with pw {}", payload.username, payload.password)
    };
    (StatusCode::OK, Json(auth_token))
}
