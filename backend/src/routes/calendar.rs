use axum::{
    extract::{State, Path},
    routing::get,
    http::StatusCode,
    Json,
    Router,
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use serde::Serialize;
use std::sync::Arc;
use sqlx;

use crate::AppState;
use crate::utils;
use crate::types::TaskDataWithId;

pub fn get_routes(state: &Arc<AppState>) -> Router {
    Router::new()
        .route("/{year}/{month}", get(get_calendar))
        .with_state(state.clone())
}

#[derive(Serialize)]
struct Calendar {
    days: Vec<Vec<TaskDataWithId>>
}

async fn get_calendar(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    State(state): State<Arc<AppState>>,
    Path((year, month)): Path<(i32, i32)>,
) -> (StatusCode, Json<Option<Calendar>>) {
    let account_id = match utils::verify_jwt(bearer.token()) {
        Some(id) => id,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };
    let all_tasks = match sqlx::query_as!(TaskDataWithId, r#"
        SELECT year, month, day,
        start_min, end_min, title, description, complete, task_id
        FROM task WHERE year=$1 AND month=$2 AND account_id=$3
        ORDER BY day;
    "#, year, month, &account_id
    ).fetch_all(&state.db_pool).await {
        Ok(rows) => rows,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Json(None));
        }
    };

    const MAX_DAYS_PER_MONTH: usize = 31;

    // Split up the results by day so that the frontend can easily render them
    let mut res: Calendar = Calendar {
        days: Vec::new()
    };
    for _ in 0..MAX_DAYS_PER_MONTH {
        res.days.push(Vec::new());
    }
    for task in all_tasks {
        // The DB should ensure task days would fit properly as indices here
        // Days are stored 1-indexed, so day 1 should be index 0
        res.days[(task.day - 1) as usize].push(task);
    }

    (StatusCode::OK, Json(Some(res)))
}
