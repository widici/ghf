mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};

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
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_data: UserData = UserData::new("muhamed").await?;
    let field = user_data.repo_data.language;

    if field.is_some() {
        println!("{}", field.unwrap());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::profile::{ProfileData, request_profile};
    use crate::api::repo::{RepoData, request_repos};

    #[tokio::test]
    async fn request_profile_works() -> Result<(), reqwest::Error> {
        let _: ProfileData = request_profile("widici").await?;
        Ok(())
    }

    #[tokio::test]
    async fn request_repos_works() -> Result<(), reqwest::Error> {
        let _: RepoData = request_repos("widici").await?;
        Ok(())
    }
}

