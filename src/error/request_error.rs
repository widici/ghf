use crate::error::error;
use error::Error;
use reqwest::header::USER_AGENT;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub async fn get_request_error(username: &str) -> Result<Error<'_>, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(&format!("https://api.github.com/users/{}", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await;

    let result = match response {
        Ok(res) => res,
        Err(_) => {
            return Ok(Error::new("Failed to handle request error", Some("Try checking your internet connection")))
        }
    };

    let ratelimit_remaining = match result.headers().get("x-ratelimit-remaining") {
        Some(header) => header.to_str()?,
        None => return Ok(Error::new("Error occurred when fetching ratelimit remaining header", None))
    };

    if ratelimit_remaining == "0" {
        let ratelimit_reset = match result.headers().get("x-ratelimit-reset") {
            Some(header) => header.to_str()?.parse::<u64>()?,
            None => return Ok(Error::new("Error occurred when fetching ratelimit reset header", None))
        };
        return handle_rate_limit(ratelimit_reset);
    }

    if result.status().is_success() {
        let description = {
            let temp = String::from(&format!("HTTP error occurred: {}", result.status()));
            Box::leak(temp.into_boxed_str())
        };
        return Ok(Error::new(description, None));
    }

    let json = match result.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(_) => {
            return Ok(Error::new("Unexpected error occurred", None))
        }
    };

    let message = match json.get("message") {
        Some(message) => {
            let temp = String::from(serde_json::to_string(message)?.trim_matches('"'));
            Box::leak(temp.into_boxed_str())
        },
        None => {
            return Ok(Error::new("Unexpected error occurred", None))
        }
    };

    if message == "Not Found" {
        return Ok(Error::new("Github user not found", Some("Try checking if the username is correct")))
    }

    Ok(Error::new(message, None))
}

fn handle_rate_limit(reset: u64) -> Result<Error<'static>, Box<dyn std::error::Error>> {
    let ratelimit_reset= UNIX_EPOCH + Duration::from_secs(reset);
    let delay = ratelimit_reset.duration_since(SystemTime::now())?.as_secs();
    let solution = {
        let temp = String::from(format!("Try again in {} minutes & {} seconds", delay/60, delay%60));
        Box::leak(temp.into_boxed_str())
    };
    return Ok(Error::new("Ratelimit exceeded", Some(solution)))
}
