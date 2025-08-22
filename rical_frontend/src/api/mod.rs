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

/// Log in and store the auth token
/// Return the auth token if successful
pub fn try_login(username: String, password: String) -> Result<String, reqwest::Error> {
    // TODO: pass around a client instead
    let client = reqwest::blocking::Client::new();
    let api_url = env::var("API_URL").expect("API_URL must be set");
    let res = client.post(format!("{api_url}/account/login"))
        .json(&Credentials {
            username,
            password
        })
        .send()?;
    let token = res.json::<LoginResult>()?.token;

    return Ok(token);
}
