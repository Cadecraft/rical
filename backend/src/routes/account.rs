use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::{Deserialize, Serialize};
use sqlx;
use std::sync::Arc;

use crate::AppState;
use crate::utils;

pub fn get_routes(state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/whoami", get(whoami))
        .with_state(state.clone())
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String,
}

async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCredentials>,
) -> StatusCode {
    let matching_username: Option<String> =
        sqlx::query_scalar("SELECT username FROM account WHERE username=$1;")
            .bind(&payload.username)
            .fetch_optional(&state.db_pool)
            .await
            .expect("Could not select username");
    if matching_username.is_some() {
        // The username is already taken
        return StatusCode::CONFLICT;
    }
    let hashed_password = utils::hash_password(&payload.password);
    let res = sqlx::query("INSERT INTO account (username, hashed_password) VALUES ($1, $2)")
        .bind(&payload.username)
        .bind(&hashed_password)
        .execute(&state.db_pool)
        .await;
    if res.is_err() {
        // TODO: better error message
        return StatusCode::BAD_REQUEST;
    }
    println!("- User signed up with username: '{}'", payload.username);
    StatusCode::CREATED
}

#[derive(Serialize)]
struct AuthToken {
    token: String,
}

struct Account {
    hashed_password: String,
    account_id: i64,
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCredentials>,
) -> (StatusCode, Json<Option<AuthToken>>) {
    let account = match sqlx::query_as!(
        Account,
        "SELECT hashed_password, account_id FROM account WHERE username=$1;",
        &payload.username
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => row,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Json(None));
        }
    };
    if !utils::verify_password(&payload.password, &account.hashed_password) {
        // TODO: better error message
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    (
        StatusCode::OK,
        Json(Some(AuthToken {
            token: utils::create_jwt(account.account_id),
        })),
    )
}

#[derive(Serialize)]
struct WhoamiResponse {
    username: String,
}

async fn whoami(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Option<WhoamiResponse>>) {
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };
    let resp = match sqlx::query_as!(
        WhoamiResponse,
        "SELECT username FROM account WHERE account_id=$1;",
        account_id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => row,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Json(None));
        }
    };
    (StatusCode::OK, Json(Some(resp)))
}
