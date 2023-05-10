mod api;
mod parsing;
mod error;

use colored::{ColoredString, Colorize};
use fields_iter::FieldsIter;
use anyhow::Result;
use std::fmt::{Display, Formatter};

use crate::api::profile::request_profile;
use crate::api::repo::request_repos;
use crate::api::image::{ImageData};
use crate::error::error::{get_error};
use crate::parsing::parse;

struct UserData<'a> {
    fields: Vec<[&'a str; 2]>,
    ascii_image: Vec<String>,
    average_color: (u8, u8, u8),
    selected_color: Option<String>,
    username: String,
}

impl<'a> UserData<'a> {
    async fn new(username: &'a str, selected_color: Option<String>) -> Result<UserData> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;

        let mut fields: Vec<[&'a str; 2]> = vec![];
        for (name, value) in FieldsIter::new(&profile_data)
            .chain(FieldsIter::new(&repo_data))
        {
            let mut field_value: String = String::from("");

            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    let inner = inner.replace("\n", " ").replace("\r", " ");
                    field_value = inner;
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                field_value = value.to_string();
            }

            if !(field_value.is_empty()) {
                let static_field_value = Box::leak(field_value.into_boxed_str());
                fields.insert(fields.len(), [name, static_field_value])
            }
        }

        let image_data: ImageData = ImageData::new(profile_data.id, fields.len() as u32 + 2).await?;
        let ascii_image = image_data.get_ascii_art()?;
        let average_color: (u8, u8, u8) = image_data.average_color();

        return Ok( UserData { fields, ascii_image, average_color, selected_color, username: profile_data.login.unwrap() } )
    }

    fn color(&self, string: &str) -> ColoredString {
        return match &self.selected_color {
            None => {
                let rgb: &(u8, u8, u8) = &self.average_color;
                string.truecolor(rgb.0, rgb.1, rgb.2)
            }
            Some(selected_color) => {
                string.color(selected_color.as_str())
            }
        }
    }
}

impl<'a> Display for UserData<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut formatted_fields: Vec<String> = self.fields
            .iter()
            .flat_map(|field| {
                let formatted_field = format!("{}: {}", &self.color(field[0]), field[1]);
                vec![formatted_field]
            })
            .collect();

        let title: String = format!("https://github.com/{}", &self.username);
        formatted_fields.splice(..0, vec![
            title.clone(), "-".repeat(title.clone().len())
        ]);


        let rows: Vec<String> = self.ascii_image
            .iter()
            .zip(formatted_fields.iter())
            .map(|(image, field)| {
                format!("{}   {}", image, field)
            })
            .collect();

        write!(f, "{}", rows.join("\n")).unwrap();

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = match parse() {
        Ok(arguments) => match arguments {
            None => { return Ok(()) }
            Some(some_args) => some_args
        }
        Err(..) => {
            eprintln!("Unexpected error occurred while parsing arguments");
            std::process::exit(1)
        }
    };

    let username: &str = args.get_one::<String>("NAME").unwrap().as_str();
    let color = args.get_one::<String>("color").map(|s|{ s.to_owned() });

    let user_data: UserData = match UserData::new(username, color).await {
        Ok(data) => data,
        Err(error) => {
            let error_obj = get_error(error, username).await?;
            eprintln!("{}", error_obj);
            std::process::exit(1);
        }
    };

    println!("{}", user_data);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use error::request_error::handle_rate_limit;
    use crate::api::request::ConfigData;
    use crate::parsing::authenticate;

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

    #[test]
    fn authenticate_test() {
        let new_token: String = String::from("token");
        let old_token = ConfigData::new().unwrap().token.unwrap();
        let auth_result = authenticate(&new_token);

        assert!(auth_result.is_ok());
        assert_eq!(new_token, ConfigData::new().unwrap().token.unwrap());

        let re_auth_result = authenticate(&old_token);
        assert!(re_auth_result.is_ok());
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
