mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let data: ProfileData = request_profile("widici").await?;
    let field = data.bio;

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

