use axum::{
    extract::{State, Path},
    routing::{get, post, patch, delete},
    http::StatusCode,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx;

use crate::AppState;
use crate::utils;

pub fn get_routes(state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/{id}", get(get_task))
        .route("/", post(post_task))
        .route("/", patch(patch_task))
        .route("/", delete(delete_task))
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

async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>
) -> (StatusCode, Json<Option<TaskData>>) {
    // TODO: AUTH!!! Get user ID from token and use as parameter
    let row: Result<(
        i32, i32, i32, Option<i32>, Option<i32>, String, Option<String>, bool
    ), sqlx::Error> = sqlx::query_as(r#"
        SELECT year, month, day,
        start_min, end_min, title, description, complete
        FROM task WHERE task_id=$1;
    "#).bind(&task_id)
        .fetch_one(&state.db_pool)
        .await;
    if row.is_err() {
        return (StatusCode::BAD_REQUEST, Json(None))
    }
    let unwrapped = row.unwrap();
    let res = TaskData {
        year: unwrapped.0,
        month: unwrapped.1,
        day: unwrapped.2,
        start_min: unwrapped.3,
        end_min: unwrapped.4,
        title: unwrapped.5,
        description: unwrapped.6,
        complete: unwrapped.7,
    };
    (StatusCode::OK, Json(Some(res)))
}

async fn post_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TaskData>
) -> (StatusCode, Json<Option<TaskData>>) {
    // TODO: AUTH!! Get user ID from token and use when constructing
    /*let row: Result<(i64,)> = sqlx::query_as(r#"
        INSERT INTO task
        ()
        VALUES
        ($1, $2, $3)
    "#).bind()*/
    (StatusCode::IM_A_TEAPOT, Json(None))
}

async fn patch_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskData>
) -> (StatusCode, Json<Option<TaskData>>) {
    (StatusCode::IM_A_TEAPOT, Json(None))
}

async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<i64>,
    Json(payload): Json<TaskData>
) -> (StatusCode, Json<Option<TaskData>>) {
    (StatusCode::IM_A_TEAPOT, Json(None))
}
