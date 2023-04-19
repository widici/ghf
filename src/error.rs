use std::fmt::{Display, Formatter};
use reqwest::header::USER_AGENT;

pub struct Error {
    pub description: &'static str,
    pub solution: Option<&'static str>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.description)?;
        if let Some(solution) = self.solution {
            writeln!(f, "{}", solution)?;
        }

        Ok(())
    }
}

impl Error {
    pub fn new(description: &'static str, solution: Option<&'static str>) -> Error {
        Error { description, solution }
    }
}

pub async fn get_error(e: Box<dyn std::error::Error>, username: &str) -> Result<Error, Box<dyn std::error::Error>> {
    return match e.downcast_ref::<reqwest::Error>() {
        Some(_error) => {
            let result = reqwest::Client::new()
                .get(&format!("https://api.github.com/users/{}", username))
                .header(USER_AGENT, "ghfetch")
                .send()
                .await?;

            if !result.status().is_success() {
                if let Ok(json) = result.json::<serde_json::Value>().await {
                    if let Some(message) = json.get("message") {
                        let description = {
                            let temp = String::from(serde_json::to_string(message).unwrap().trim_matches('"'));
                            Box::leak(temp.into_boxed_str())
                        };
                        Ok(Error { description, solution: None })
                    } else {
                        Ok(Error::new("Unexpected error occurred", None))
                    }
                } else {
                    Ok(Error::new("Unexpected error occurred", None))
                }
            } else {
                Ok(Error::new("An unexpected error occurred", None))
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
