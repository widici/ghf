use crate::error::request_error;
use request_error::get_request_error;

use std::fmt::{Display, Formatter};
use colored::Colorize;

pub struct Error<'a> {
    pub description: &'a str,
    pub solution: Option<&'a str>,
}


impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.description.color("red"))?;
        if let Some(solution) = self.solution {
            writeln!(f, "{}", solution.color("red"))?;
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
    return if e.is::<reqwest::Error>() {
        get_request_error(username).await
    } else if e.is::<serde_json::Error>() {
        Ok(Error::new("An error occurred when deserializing the user data", None))
    } else if e.is::<image::ImageError>() {
        Ok(Error::new("An error occurred while processing the Github avatar", None))
    } else {
        Ok(Error::new("An unexpected error occurred", None))
    }
}
