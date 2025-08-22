use reqwest;
use std::env;
use serde::{Serialize, Deserialize};

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
        // TODO: pass around a client instead
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
}
