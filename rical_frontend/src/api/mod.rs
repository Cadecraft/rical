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
    auth_token: Option<String>,
    cached_calendar_tasks: Option<types::CalendarTasks>,
}

pub enum CacheType {
    PreferCache,
    Refresh
}

impl CacheType {
    pub fn prefer_cache(&self) -> bool {
        match self {
            CacheType::PreferCache => true,
            CacheType::Refresh => false
        }
    }
}

impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {
            blocking_client: reqwest::blocking::Client::new(),
            auth_token: None,
            cached_calendar_tasks: None,
        }
    }

    fn api_url() -> String {
        // TODO: get a different way, such as via a config file?
        env::var("API_URL").expect("API_URL must be set")
    }

    fn expect_auth_token(&self) -> String {
        self.auth_token.clone().expect("Must be logged in to perform this action")
    }

    /// Log in and store the auth token
    pub fn try_login(&mut self, username: String, password: String) -> Result<(), reqwest::Error> {
        let res = self.blocking_client.post(format!("{}/account/login", Self::api_url()))
            .json(&Credentials {
                username,
                password
            })
            .send()?;
        let token = res.json::<LoginResult>()?.token;

        self.auth_token = Some(token.clone());

        Ok(())
    }

    /// Sign up a new account
    pub fn try_signup(&mut self, username: String, password: String) -> Result<(), reqwest::Error> {
        let res = self.blocking_client.post(format!("{}/account/signup", Self::api_url()))
            .json(&Credentials {
                username,
                password
            })
            .send()?;
        res.error_for_status()?;

        Ok(())
    }

    pub fn fetch_tasks_at_date(&mut self, date: &utils::RicalDate, cache_type: CacheType) -> Vec<types::TaskDataWithId> {
        let calendar_tasks = self.fetch_calendar_tasks(date.year, date.month, cache_type);
        let empty_res: Vec<types::TaskDataWithId> = vec![];
        calendar_tasks.days.get(date.day as usize - 1).unwrap_or(&empty_res).clone()
    }

    /// Fetch a calendar from the API. If this year/month calendar was already fetched, just return that one
    /// Only calling this method with `CacheType::PreferCache` could lead to data being out of sync
    pub fn fetch_calendar_tasks(&mut self, year: i32, month: u32, cache_type: CacheType) -> types::CalendarTasks {
        // TODO: cache based on year and month, rather than just based on whether this was called
        if cache_type.prefer_cache() && self.cached_calendar_tasks.is_some() {
            return self.cached_calendar_tasks.clone().unwrap();
        }

        let res = self.blocking_client.get(format!("{}/calendar/{}/{}", Self::api_url(), year, month))
            .bearer_auth(self.expect_auth_token())
            .send().unwrap();

        let calendar_tasks = res.json::<types::CalendarTasks>().unwrap();
        self.cached_calendar_tasks = Some(calendar_tasks.clone());

        calendar_tasks

        // TODO: this is just dummy data; actually call the api
        // TODO: caching
        /*let dummy_task = types::TaskDataWithId {
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
        }*/
    }
}
