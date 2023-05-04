use anyhow::Result;
use std::fs::read_to_string;
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    token: Option<String>,
    user: String,
}

impl Config {
    fn new() -> Result<Config> {
        let json = read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&json)?;

        return Ok(config)
    }
}

pub async fn request(url: &str) -> Result<reqwest::Response> {
    let config = Config::new()?;

    return if let Some(token) = config.token {
        Ok(request_with_token(&token, &config.user, url).await?)
    } else {
        Ok(request_without_token(&config.user, url).await?)
    }
}

async fn request_with_token(token: &str, user: &str, url: &str) -> Result<reqwest::Response> {
    return Ok(reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, format!("ghfetch ({})", user))
        .header(AUTHORIZATION, token)
        .send()
        .await?)
}

async fn request_without_token(user: &str, url: &str) -> Result<reqwest::Response> {
    return Ok(reqwest::Client::new()
        .get(url)
        .header(USER_AGENT, format!("ghfetch ({})", user))
        .send()
        .await?)
}