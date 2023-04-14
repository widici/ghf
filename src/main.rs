mod api;
mod error;
mod parsing;

use colored::Colorize;
use fields_iter::FieldsIter;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
use crate::api::image::{ImageData};
use crate::error::handle_error;
use crate::parsing::parse;

struct UserData {
    profile_data: ProfileData,
    repo_data: RepoData,
    image_data: ImageData,
    color: (u8, u8, u8)
}


impl UserData {
    async fn new(username: &str) -> Result<UserData, reqwest::Error> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;
        let image_data = ImageData::new(profile_data.id).await?;
        let color: (u8, u8, u8) = image_data.average_color();

        return Ok( UserData { profile_data, repo_data, image_data, color } )
    }

    async fn display(&self) -> Result<(), reqwest::Error> {
        let mut fields: Vec<String> = Vec::new();
        let color = self.color;

        let title: String = format!("https://github.com/{}", &self.profile_data.login.as_ref().unwrap());
        let dashes: String = "-".repeat(title.len());
        fields.append(&mut vec![title, dashes]);

        for (name, value) in FieldsIter::new(&self.profile_data)
            .chain(FieldsIter::new(&self.repo_data))
        {
            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    let inner = inner.replace("\n", " ").replace("\r", " ");
                    fields.insert(fields.len(), format!("{}: {}", name.truecolor(color.0, color.1, color.2), inner));
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                fields.insert(fields.len(), format!("{}: {}", name.truecolor(color.0, color.1, color.2), value));
            }
        }

        let mut rows = self.image_data.get_ascii_art(fields.len() as u32)?;

        for field in fields {
            println!("{}   {}", rows.remove(0), field)
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = match parse() {
        Ok(arguments) => arguments,
        Err(..) => {
            eprintln!("Error occurred while parsing arguments");
            std::process::exit(1)
        }
    };

    let username: &str = args.value_of("name").unwrap_or("widici");

    let user_data: UserData = match UserData::new(username).await {
        Ok(data) => data,
        Err(..) => {
            handle_error(username).await?;
            std::process::exit(1);
        }
    };

    user_data.display().await.unwrap();

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn requests_works() -> Result<(), reqwest::Error> {
        let _: UserData = UserData::new("widici").await?;
        Ok(())
    }
}
