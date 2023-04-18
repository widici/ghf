use reqwest::header::USER_AGENT;

pub async fn handle_error(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = reqwest::Client::new()
        .get(&format!("https://api.github.com/users/{}", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await?;

    if !result.status().is_success() {
        if let Ok(json) = result.json::<serde_json::Value>().await {
            if let Some(message) = json.get("message") {
                eprintln!("{}", serde_json::to_string(message).unwrap().trim_matches('"'))
            }
        } else {
            eprintln!("Unexpected error occurred")
        }
    }

    Ok(())
}