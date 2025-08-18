use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    Json,
    Router
};

use crate::AppState;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[derive(Serialize)]
struct AuthToken {
    token: String
}

async fn signup(Json(payload): Json<UserCredentials>) -> (StatusCode, Json<()>) {
    // TODO: insert into db
    // TODO: hash/salt password
    // TODO: return whether successful (or fail)
    /*let data = state.data.lock().expect("mutex was poisoned");
    let d = *data;
    if .get_mut().temporary_testing_users.contains_key(&payload.username) {
        return (StatusCode::BAD_REQUEST, Json(()));
    }*/
    println!("User signed up with username: '{}'", payload.username);
    /*data.data.get_mut().temporary_testing_users.insert(payload.username, payload.password);*/
    (StatusCode::CREATED, Json(()))
}

async fn login(Json(payload): Json<UserCredentials>) -> (StatusCode, Json<AuthToken>) {
    // TODO: check against db, return response with auth token, etc.
    let auth_token = AuthToken {
        token: format!("log in example: {} with pw {}", payload.username, payload.password)
    };
    (StatusCode::OK, Json(auth_token))
}
