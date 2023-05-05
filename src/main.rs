mod api;
mod parsing;
mod error;
mod util;

use colored::{ColoredString, Colorize};
use fields_iter::FieldsIter;
use anyhow::Result;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
use crate::api::image::{ImageData};
use crate::error::error::{get_error};
use crate::parsing::parse;

struct UserData {
    profile_data: ProfileData,
    repo_data: RepoData,
    image_data: ImageData,
    average_color: (u8, u8, u8),
    selected_color: Option<String>
}

impl UserData {
    async fn new(username: &str, selected_color: Option<String>) -> Result<UserData> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;
        let image_data = ImageData::new(profile_data.id).await?;
        let average_color: (u8, u8, u8) = image_data.average_color();

        return Ok( UserData { profile_data, repo_data, image_data, average_color, selected_color} )
    }

    async fn display(&self) -> Result<()> {
        let mut fields: Vec<String> = Vec::new();

        let title: String = format!("https://github.com/{}", &self.profile_data.login.as_ref().unwrap());
        let dashes: String = "-".repeat(title.len());
        fields.append(&mut vec![title, dashes]);

        for (name, value) in FieldsIter::new(&self.profile_data)
            .chain(FieldsIter::new(&self.repo_data))
        {
            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    let inner = inner.replace("\n", " ").replace("\r", " ");
                    fields.insert(fields.len(), format!("{}: {}", &self.color(name), inner));
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                fields.insert(fields.len(), format!("{}: {}", &self.color(name), value));
            }
        }

        let mut rows = self.image_data.get_ascii_art(fields.len() as u32)?;

        for field in fields {
            println!("{}   {}", rows.remove(0), field)
        }

        Ok(())
    }

    fn color(&self, string: &str) -> ColoredString {
        return match &self.selected_color {
            None => {
                let rgb: &(u8, u8, u8) = &self.average_color;
                string.truecolor(rgb.0, rgb.1, rgb.2)
            }
            Some(selected_color) => {
                string.color(&**selected_color)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = match parse() {
        Ok(arguments) => arguments,
        Err(..) => {
            eprintln!("Unexpected error occurred while parsing arguments");
            std::process::exit(1)
        }
    };

    if let Some(("auth", auth_args)) = args.subcommand() {
        let token = auth_args.get_one::<String>("TOKEN").unwrap();
        println!("{}", token);
        return Ok(())
    }

    let username: &String = args.get_one::<String>("NAME").unwrap();
    let color = args.get_one::<String>("color").map(|s|{ s.to_owned() });

    let user_data: UserData = match UserData::new(&*username, color).await {
        Ok(data) => data,
        Err(error) => {
            let error_obj = get_error(error, &*username).await?;
            eprintln!("{}", error_obj);
            std::process::exit(1);
        }
    };

    user_data.display().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::*;
    use error::request_error::handle_rate_limit;

    #[tokio::test]
    async fn requests_works() {
        let result = UserData::new("widici", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn reqwest_error_test() {
        let error = anyhow::Error::new(reqwest::get("https://nonexistenturl.com").await.unwrap_err());
        let result = get_error(error, "widici").await;
        assert!(result.is_ok());
    }

    #[test]
    fn handle_rate_limit_test() {
        let reset = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 30;
        let result = handle_rate_limit(reset).unwrap();
        let seconds = (result.solution.unwrap().split(" ").collect::<Vec<&str>>()[6]
            .parse::<f32>().unwrap() / 10.0).round() * 10.0;

        assert_eq!(result.description, "Ratelimit exceeded");
        assert!(result.solution.unwrap().starts_with("Try again in 0 minutes &"));
        assert_eq!(seconds, 30 as f32)
    }

    /*
    #[tokio::test]
    async fn serde_error() {
        let error = anyhow::Error::new(serde_json::from_str("Invalid json"));
        let result = get_error(error, "widici").await.unwrap();
        assert_eq!(result.description, "An error occurred when deserializing the user data")
    }
    */
}
