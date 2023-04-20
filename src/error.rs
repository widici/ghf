use std::fmt::{Display, Formatter};
use reqwest::header::USER_AGENT;

pub struct Error<'a> {
    pub description: &'a str,
    pub solution: Option<&'a str>,
}


impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.description)?;
        if let Some(solution) = self.solution {
            writeln!(f, "{}", solution)?;
        }

        Ok(())
    }
}

impl<'a> Error<'a> {
    pub fn new(description: &'a str, solution: Option<&'a str>) -> Error<'a> {
        Error { description, solution }
    }
}

pub async fn get_error(e: Box<dyn std::error::Error>, username: &str) -> Result<Error<'_>, Box<dyn std::error::Error>> {
    return match e.downcast_ref::<reqwest::Error>() {
        Some(_error) => {
            let result = reqwest::Client::new()
                .get(&format!("https://api.github.com/users/{}", username))
                .header(USER_AGENT, "ghfetch")
                .send()
                .await;

            let result = match result {
                Ok(res) => res,
                Err(_) => {
                    return Ok(Error::new("Failed to handle request error", Some("Try checking your internet connection")))
                }
            };

            if !result.status().is_success() {
                if let Ok(json) = result.json::<serde_json::Value>().await {
                    if let Some(message) = json.get("message") {
                        let description = {
                            let temp = String::from(serde_json::to_string(message).unwrap().trim_matches('"'));
                            Box::leak(temp.into_boxed_str())
                        };
                        Ok(Error::new(description, None))
                    } else {
                        Ok(Error::new("Unexpected error occurred", None)) }
                } else {
                    Ok(Error::new("Unexpected error occurred", None)) }
            } else {
                let description = {
                    let temp = String::from(&format!("HTTP error occurred: {}", result.status()));
                    Box::leak(temp.into_boxed_str())
                };
                Ok(Error::new(description, None))
            }
        },
        None => {
            if e.is::<serde_json::Error>() {
                Ok(Error::new("An error occurred when deserializing the user data", None))
            } else if e.is::<image::ImageError>() {
                Ok(Error::new("An error occurred while processing the Github avatar", None))
            } else {
                Ok(Error::new("An unexpected error occurred", None))
            }
        }
    };
}
