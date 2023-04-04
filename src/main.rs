mod api;

use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
use fields_iter::FieldsIter;

struct UserData {
    profile_data: ProfileData,
    repo_data: RepoData,
}

impl Display for UserData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (name, value) in FieldsIter::new(&self.profile_data) 
            .chain(FieldsIter::new(&self.repo_data))
        {
            if let Some(value) = value.downcast_ref::<Option<String>>() {
                if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                    writeln!(f, "{}: {}", name.color("red"), inner).unwrap();
                }
            } else if let Some(value) = value.downcast_ref::<i32>() {
                writeln!(f, "{}: {}", name.color("red"), value).unwrap();
            }
        }
        Ok(())
    }
}

impl UserData {
    async fn new(username: &str) -> Result<UserData, reqwest::Error> {
        let profile_data= request_profile(username).await?;
        let repo_data = request_repos(username).await?;

        return Ok( UserData { profile_data, repo_data } )
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_data: UserData = UserData::new("widici").await?;
    println!("{}", user_data);

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
        let _ = request_image(84205124);
        Ok(())
    }
}
