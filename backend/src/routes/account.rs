use axum::{
    extract::State,
    routing::post,
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
) -> (StatusCode, Json<Option<AuthToken>>) {
    let row: Result<(String, i64), sqlx::Error> = sqlx::query_as(
        "SELECT hashed_password, account_id FROM account WHERE username=$1;"
    ).bind(&payload.username)
        .fetch_one(&state.db_pool)
        .await;
    if row.is_err() {
        // TODO: better error message
        return (StatusCode::BAD_REQUEST, Json(None));
    }
    let rowres = row.unwrap();
    let stored_password = rowres.0;
    let stored_account_id = rowres.1;
    let valid = utils::verify_password(&payload.password, &stored_password);
    if !valid {
        // TODO: better error message
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    (StatusCode::OK, Json(Some(AuthToken {
        token: utils::create_jwt(stored_account_id)
    })))
}
