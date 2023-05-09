use anyhow::Result;
use std::fs::read_to_string;
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use crate::error::error::Error;

#[derive(Deserialize, Serialize)]
pub struct ConfigData {
    pub token: Option<String>,
    pub user: String,
}

impl ConfigData {
    pub fn new() -> Result<ConfigData> {
        let json = read_to_string("config.json")?;
        let config: ConfigData = serde_json::from_str(&json)?;

        return Ok(config)
    }
}

pub async fn request(url: &str) -> Result<Response> {
    let config = ConfigData::new()?;

    return if let Some(token) = config.token {
        Ok(request_with_token(&token, &config.user, url).await?)
    } else {
        Ok(request_without_token(&config.user, url).await?)
    }
}

async fn request_with_token(token: &str, user: &str, url: &str) -> Result<Response> {
    let response = reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, format!("ghf ({})", user))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await?;

    if [StatusCode::UNAUTHORIZED, StatusCode::FORBIDDEN].contains(&response.status()) {
        eprintln!("{}", Error::new("Failed to authenticate to the GitHub api", Some("Try checking the authentication token")));
        std::process::exit(1)
    } else {
        return Ok(response)
    }
}

async fn request_without_token(user: &str, url: &str) -> Result<Response> {
    return Ok(reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, format!("ghf ({})", user))
        .send()
        .await?)
}