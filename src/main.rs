mod api;

use colored::Colorize;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
use crate::api::image::request_image;
use fields_iter::FieldsIter;

struct UserData {
    profile_data: ProfileData,
    repo_data: RepoData,
}


impl UserData {
    async fn new(username: &str) -> Result<UserData, reqwest::Error> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;

        return Ok( UserData { profile_data, repo_data } )
    }

    async fn display(self) -> Result<(), reqwest::Error> {
        let mut fields: Vec<String> = Vec::new();
        for (name, value) in FieldsIter::new(&self.profile_data)
            .chain(FieldsIter::new(&self.repo_data))
        {
            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    fields.insert(fields.len(), format!("{}: {}", name.color("cyan"), inner));
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                fields.insert(fields.len(), format!("{}: {}", name.color("cyan"), value));
            }
        }

        let mut rows = request_image(self.profile_data.id, fields.len() as u32).await.unwrap();

        for field in fields {
            println!("{}   {}", rows.remove(0), field)
        }

        for row in rows {
            println!("{}", row)
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_data: UserData = UserData::new("widici").await?;
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
