use axum::{
    extract::{State, Path},
    routing::{get, post, patch, delete},
    http::StatusCode,
    Json,
    Router,
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx;

use crate::AppState;
use crate::utils;

pub fn get_routes(state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/{id}", get(get_task))
        .route("/", post(post_task))
        .route("/{id}", patch(patch_task))
        .route("/{id}", delete(delete_task))
        .with_state(state.clone())
}

#[derive(Deserialize, Serialize)]
struct TaskData {
    year: i32,
    month: i32,
    day: i32,
    start_min: Option<i32>,
    end_min: Option<i32>,
    title: String,
    description: Option<String>,
    complete: bool
}

#[derive(Deserialize, Serialize)]
struct TaskId {
    task_id: i64
}

async fn get_task(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>
) -> (StatusCode, Json<Option<TaskData>>) {
    // TODO: refactor into middleware?
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };
    let res = match sqlx::query_as!(TaskData, r#"
        SELECT year, month, day,
        start_min, end_min, title, description, complete
        FROM task WHERE task_id=$1 AND account_id=$2;
    "#, &task_id, &account_id
    ).fetch_one(&state.db_pool).await {
        Ok(row) => row,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Json(None));
        }
    };

    (StatusCode::OK, Json(Some(res)))
}

async fn post_task(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskData>,
) -> (StatusCode, Json<Option<TaskId>>) {
    // TODO: refactor into middleware?
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };
    let task_id = match sqlx::query_as!(TaskId, r#"
        INSERT INTO task
        (account_id, year, month, day, start_min, end_min, title, description, complete)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING task_id
    "#, account_id, payload.year, payload.month, payload.day, payload.start_min, payload.end_min, payload.title, payload.description, payload.complete).fetch_one(&state.db_pool).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };
    (StatusCode::CREATED, Json(Some(task_id)))
}

async fn patch_task(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskData>
) -> (StatusCode, Json<Option<TaskData>>) {
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };
    let res = match sqlx::query_as!(TaskData, r#"
        UPDATE task
        SET year = $1, month = $2, day = $3, start_min = $4, end_min = $5, title = $6,
            description = $7, complete = $8
        WHERE task_id = $9 AND account_id = $10
        RETURNING year, month, day, start_min, end_min, title, description, complete
    "#, payload.year, payload.month, payload.day, payload.start_min, payload.end_min, payload.title, payload.description, payload.complete, task_id, account_id).fetch_one(&state.db_pool).await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };
    (StatusCode::OK, Json(Some(res)))
}

async fn delete_task(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskId>
) -> StatusCode {
    // TODO: remove unnecessary payload without causing errors
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return StatusCode::UNAUTHORIZED;
        }
    };
    match sqlx::query_as!(TaskData, r#"
        DELETE FROM task
        WHERE task_id = $1 AND account_id = $2
    "#, task_id, account_id).fetch_one(&state.db_pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST
    }
}
