use serde::{Serialize, Deserialize};

// NOTE: some of these types are copied from the backend
// NOTE: May want to look into a better long-term type sharing solution

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskData {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub start_min: Option<i32>,
    pub end_min: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub complete: bool
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskDataWithId {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub start_min: Option<i32>,
    pub end_min: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub complete: bool,
    pub task_id: i64
}

impl TaskDataWithId {
    pub fn duration_mins(&self) -> Option<i32> {
        if self.start_min.is_some() && self.end_min.is_some() {
            Some(self.end_min.unwrap() - self.start_min.unwrap())
        } else {
            None
        }
    }

    pub fn without_id(&self) -> TaskData {
        TaskData {
            year: self.year,
            month: self.month,
            day: self.day,
            start_min: self.start_min,
            end_min: self.end_min,
            title: self.title.clone(),
            description: self.description.clone(),
            complete: self.complete
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CalendarTasks {
    pub days: Vec<Vec<TaskDataWithId>>
}
