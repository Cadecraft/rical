use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use crate::types;
use crate::utils;

#[derive(Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResult {
    token: String,
}

pub enum CacheType {
    /// If the matching parameters are found in the cache, use that instead of calling the API
    PreferCache,
    /// Call the API with these parameters and update the cache with the new results
    RefreshOne,
}

pub struct ApiHandler {
    blocking_client: reqwest::blocking::Client,
    auth_token: Option<String>,
    cached_calendar_tasks: HashMap<(i32, i32), types::CalendarTasks>,
}

impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {
            blocking_client: reqwest::blocking::Client::new(),
            auth_token: None,
            cached_calendar_tasks: HashMap::new(),
        }
    }

    fn api_url() -> String {
        // TODO: get a different way, such as via a config file?
        env::var("API_URL").expect("API_URL must be set")
    }

    fn expect_auth_token(&self) -> String {
        self.auth_token
            .clone()
            .expect("Must be logged in to perform this action")
    }

    /// Log in and store the auth token
    pub fn try_login(&mut self, username: String, password: String) -> Result<(), reqwest::Error> {
        let res = self
            .blocking_client
            .post(format!("{}/account/login", Self::api_url()))
            .json(&Credentials { username, password })
            .send()?;
        let token = res.json::<LoginResult>()?.token;

        self.auth_token = Some(token.clone());

        Ok(())
    }

    /// Sign up a new account
    pub fn try_signup(&mut self, username: String, password: String) -> Result<(), reqwest::Error> {
        let res = self
            .blocking_client
            .post(format!("{}/account/signup", Self::api_url()))
            .json(&Credentials { username, password })
            .send()?;
        res.error_for_status()?;

        Ok(())
    }

    pub fn fetch_tasks_at_date(
        &mut self,
        date: &utils::RicalDate,
        cache_type: CacheType,
    ) -> Vec<types::TaskDataWithId> {
        let calendar_tasks = self.fetch_calendar_tasks(date.year, date.month as i32, cache_type);
        let empty_res: Vec<types::TaskDataWithId> = vec![];
        calendar_tasks
            .days
            .get(date.day as usize - 1)
            .unwrap_or(&empty_res)
            .clone()
    }

    /// Fetch a calendar from the API. If this year/month calendar was already fetched, just return that one
    /// Only calling this method with `CacheType::PreferCache` could lead to data being out of sync
    pub fn fetch_calendar_tasks(
        &mut self,
        year: i32,
        month: i32,
        cache_type: CacheType,
    ) -> types::CalendarTasks {
        let identifier = (year, month);
        match cache_type {
            CacheType::PreferCache => match self.cached_calendar_tasks.get(&identifier) {
                Some(cached) => {
                    return cached.clone();
                }
                None => (),
            },
            CacheType::RefreshOne => (),
        }

        let res = self
            .blocking_client
            .get(format!("{}/calendar/{}/{}", Self::api_url(), year, month))
            .bearer_auth(self.expect_auth_token())
            .send()
            .unwrap();

        let calendar_tasks = res.json::<types::CalendarTasks>().unwrap();
        self.cached_calendar_tasks
            .insert(identifier, calendar_tasks.clone());

        calendar_tasks
    }

    /// Post a task and refresh the calendar data from the API accordingly
    pub fn post_new_task(&mut self, task: &types::TaskData) -> Result<(), reqwest::Error> {
        let res = self
            .blocking_client
            .post(format!("{}/task", Self::api_url()))
            .bearer_auth(self.expect_auth_token())
            .json(&task)
            .send()?;
        res.error_for_status()?;

        self.fetch_calendar_tasks(task.year, task.month, CacheType::RefreshOne);

        Ok(())
    }

    /// Update an existing task and refresh the calendar accordingly; return whether the date changed
    pub fn update_task(&mut self, task: &types::TaskDataWithId) -> Result<bool, reqwest::Error> {
        let res = self
            .blocking_client
            .put(format!("{}/task/{}", Self::api_url(), task.task_id))
            .bearer_auth(self.expect_auth_token())
            .json(&task.without_id())
            .send()?;
        let res = res.error_for_status()?;
        let original = res.json::<types::TaskData>().unwrap();

        // Must update the previously designated month AND the newly designated month if both have changed
        self.fetch_calendar_tasks(original.year, original.month, CacheType::RefreshOne);
        let calendar_frame_changed = original.year != task.year || original.month != task.month;
        if calendar_frame_changed {
            self.fetch_calendar_tasks(task.year, task.month, CacheType::RefreshOne);
        }
        let date_changed = calendar_frame_changed || original.day != task.day;

        Ok(date_changed)
    }

    /// Toggle whether a task is completed and refresh the calendar accordingly
    pub fn toggle_completed(&mut self, task: &types::TaskDataWithId) -> Result<(), reqwest::Error> {
        let mut updated = task.clone();
        updated.complete = !updated.complete;
        self.update_task(&updated)?;
        self.fetch_calendar_tasks(task.year, task.month, CacheType::RefreshOne);

        Ok(())
    }

    /// Delete a task and refresh the calendar accordingly
    pub fn delete_task(&mut self, task: &types::TaskDataWithId) -> Result<(), reqwest::Error> {
        let res = self
            .blocking_client
            .delete(format!("{}/task/{}", Self::api_url(), task.task_id))
            .bearer_auth(self.expect_auth_token())
            .send()?;
        res.error_for_status()?;

        self.fetch_calendar_tasks(task.year, task.month, CacheType::RefreshOne);

        Ok(())
    }
}
