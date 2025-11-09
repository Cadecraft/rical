use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskData {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub start_min: Option<i32>,
    pub end_min: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub complete: bool,
}

#[derive(Deserialize, Serialize)]
pub struct TaskId {
    pub task_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct TaskDataWithId {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub start_min: Option<i32>,
    pub end_min: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub complete: bool,
    pub task_id: i64,
}
