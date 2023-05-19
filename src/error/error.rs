use crate::error::request_error;
use request_error::get_request_error;

use std::fmt::{Display, Formatter};
use colored::Colorize;
use anyhow::Result;

pub struct Error<'a> {
    pub description: &'a str,
    pub solution: Option<&'a str>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.description.color("red"))?;
        if let Some(solution) = self.solution {
            write!(f, "\n{}", solution.color("red"))?;
        }

        Ok(())
    }
}

impl<'a> Error<'a> {
    pub fn new(description: &'a str, solution: Option<&'a str>) -> Error<'a> {
        Error { description, solution }
    }
}

pub async fn get_error(e: anyhow::Error, username: &str) -> Result<Error<'_>> {
    return if e.is::<reqwest::Error>() {
        get_request_error(username).await
    } else if e.is::<serde_json::Error>() {
        Ok(Error::new("An error occurred when deserializing the user data", None))
    } else if e.is::<image::ImageError>() {
        Ok(Error::new("An error occurred while processing the Github avatar", None))
    } else {
        let description = {
            let temp: String = format!("An unexpected error occurred: {:?}", e);
            Box::leak(temp.into_boxed_str())
        };
        return Ok(Error::new(description, None))
    }

    /* Uses an unstable feature std::any::type_value_of_val()
    return match std::any::type_name_of_val(&e) {
        "reqwest::Error" => get_request_error(username).await,
        "serde_json::Error" => Ok(Error::new("An error occurred when deserializing the user data", None)),
        "image::ImageError" => Ok(Error::new("An error occurred while processing the Github avatar", None)),
        &_ => Ok(Error::new("An unexpected error occurred", None))
    }
    */
}
