use reqwest;
use std::env;
use serde::{Serialize, Deserialize};

use crate::utils;
use crate::types;

#[derive(Serialize)]
struct Credentials {
    username: String,
    password: String
}

#[derive(Deserialize)]
struct LoginResult {
    token: String
}

pub struct ApiHandler {
    blocking_client: reqwest::blocking::Client,
    auth_token: Option<String>
}

impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {
            blocking_client: reqwest::blocking::Client::new(),
            auth_token: None
        }
    }

    /// Log in and store the auth token
    /// Return the auth token if successful
    pub fn try_login(&mut self, username: String, password: String) -> Result<String, reqwest::Error> {
        let api_url = env::var("API_URL").expect("API_URL must be set");
        let res = self.blocking_client.post(format!("{api_url}/account/login"))
            .json(&Credentials {
                username,
                password
            })
            .send()?;
        let token = res.json::<LoginResult>()?.token;

        self.auth_token = Some(token.clone());

        return Ok(token);
    }

    /// Sign up a new account
    /// Return the new username if successful
    pub fn try_signup(&mut self, username: String, password: String) -> Result<(), reqwest::Error> {
        let api_url = env::var("API_URL").expect("API_URL must be set");
        let res = self.blocking_client.post(format!("{api_url}/account/signup"))
            .json(&Credentials {
                username,
                password
            })
            .send()?;

        res.error_for_status()?;

        return Ok(());
    }

    pub fn fetch_tasks_at_date_cached(&mut self, date: &utils::RicalDate) -> Vec<types::TaskDataWithId> {
        let calendar_tasks = self.fetch_calendar_tasks_cached(date.year, date.month);
        let empty_res: Vec<types::TaskDataWithId> = vec![];
        calendar_tasks.days.get(date.day as usize - 1).unwrap_or(&empty_res).clone()
    }

    /// Fetch a calendar from the API. If this year/month calendar was already fetched, just return that one
    /// Only using this method could lead to data being out of sync
    pub fn fetch_calendar_tasks_cached(&mut self, year: i32, month: u32) -> types::CalendarTasks {
        // TODO: this is just dummy data; actually call the api
        // TODO: caching
        let dummy_task = types::TaskDataWithId {
            year: 2025,
            month: 8,
            day: 7,
            start_min: None,
            end_min: None,
            title: "Test".to_string(),
            description: None,
            complete: false,
            task_id: 3
        };
        let dummy_task_2 = types::TaskDataWithId {
            year: 2025,
            month: 8,
            day: 7,
            start_min: Some(360),
            end_min: Some(450),
            title: "Test 2".to_string(),
            description: None,
            complete: false,
            task_id: 4
        };
        let dummy_task_3 = types::TaskDataWithId {
            year: 2025,
            month: 8,
            day: 7,
            start_min: Some(480),
            end_min: None,
            title: "Test 3".to_string(),
            description: None,
            complete: true,
            task_id: 5
        };
        let dummy_task_4 = types::TaskDataWithId {
            year: 2025,
            month: 8,
            day: 7,
            start_min: Some(480),
            end_min: Some(497),
            title: "Test 4".to_string(),
            description: None,
            complete: true,
            task_id: 6
        };
        types::CalendarTasks {
            days: vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![
                    dummy_task_2.clone(),
                    dummy_task_3.clone(),
                    dummy_task.clone(),
                ],
                vec![
                    dummy_task_2.clone(),
                    dummy_task.clone(),
                ],
                vec![],
                vec![
                    dummy_task_3.clone(),
                    dummy_task.clone(),
                ],
                vec![],
                vec![
                    dummy_task.clone(),
                ],
                vec![],
                vec![],
                vec![],
                vec![
                    dummy_task.clone(),
                    dummy_task_2.clone(),
                    dummy_task_3.clone(),
                    dummy_task_4.clone(),
                ],
                vec![],
                vec![],
                vec![],
                vec![],
            ]
        }
    }
}
