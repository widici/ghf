use reqwest::header::USER_AGENT;

pub struct Error {
    pub description: &'static str,
    pub solution: Option<&'static str>,
}

pub async fn get_error(error: Box<dyn std::error::Error>, username: &str) -> Result<Error, Box<dyn std::error::Error>> {
    if let Some(_) = error.downcast_ref::<reqwest::Error>() {
        let result = reqwest::Client::new()
            .get(&format!("https://api.github.com/users/{}", username))
            .header(USER_AGENT, "ghfetch")
            .send()
            .await?;

        if !result.status().is_success() {
            return if let Ok(json) = result.json::<serde_json::Value>().await {
                if let Some(message) = json.get("message") {
                    let description = {
                        let temp = String::from(serde_json::to_string(message).unwrap().trim_matches('"'));
                        Box::leak(temp.into_boxed_str())
                    };
                    Ok( Error { description, solution: None } )
                } else {
                    Ok( Error { description: "Unexpected error occurred", solution: None } )
                }
            } else {
                Ok( Error { description: "Unexpected error occurred", solution: None } )
            }
        }
    } else if let Some(_) = error.downcast_ref::<image::ImageError>() {
        return Ok( Error { description: "An error occurred while processing the Github avatar", solution: None } );
    } else if let Some(_) = error.downcast_ref::<serde_json::Error>() {
        return Ok( Error { description: "An error occurred when deserializing the user data", solution: None } );
    }

    return Ok( Error { description: "Unexpected error occurred", solution: None } );
}
