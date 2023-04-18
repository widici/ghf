use reqwest::header::USER_AGENT;

pub async fn handle_error(error: Box<dyn std::error::Error>, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_) = error.downcast_ref::<reqwest::Error>() {
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
    } else if let Some(_) = error.downcast_ref::<image::ImageError>() {
        eprintln!("An error occurred while processing the Github avatar")
    } else if let Some(_) = error.downcast_ref::<serde_json::Error>() {
        eprintln!("An error occurred when deserializing the user data")
    } else {
        eprintln!("Unexpected error occurred")
    }

    // An error occurred while parsing the user's data.
    // Check if the user's name is correct.

    Ok(())
}