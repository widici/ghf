mod api;
mod error;

use colored::Colorize;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
use crate::api::image::{ImageData};
use crate::error::handle_error;
use fields_iter::FieldsIter;

struct UserData {
    profile_data: ProfileData,
    repo_data: RepoData,
    image_data: ImageData,
}


impl UserData {
    async fn new(username: &str) -> Result<UserData, reqwest::Error> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;
        let image_data = ImageData::new(profile_data.id).await?;

        return Ok( UserData { profile_data, repo_data, image_data } )
    }

    async fn display(&self) -> Result<(), reqwest::Error> {
        let mut fields: Vec<String> = Vec::new();

        let title: String = format!("https://github.com/{}", &self.profile_data.login.as_ref().unwrap());
        let dashes: String = "-".repeat(title.len());
        fields.append(&mut vec![title, dashes]);

        for (name, value) in FieldsIter::new(&self.profile_data)
            .chain(FieldsIter::new(&self.repo_data))
        {
            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    fields.insert(fields.len(), format!("{}: {}", name.color("red"), inner));
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                fields.insert(fields.len(), format!("{}: {}", name.color("red"), value));
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
    let username: &str = "imnotauserongithub";
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
    use crate::{UserData};
    use crate::api::image::request_image;

    #[tokio::test]
    async fn requests_works() -> Result<(), reqwest::Error> {
        let _: UserData = UserData::new("widici").await?;
        Ok(())
    }

    #[test]
    fn request_image_works() -> Result<(), reqwest::Error> {
        let _ = request_image(84205124, 15 as u32);
        Ok(())
    }
}
