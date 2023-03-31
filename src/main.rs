mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let data: Vec<RepoData> = request_repos("widici").await?;
    for repo in data {
        if repo.forks.is_some() {
            println!("{}", repo.forks.unwrap())
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::profile::{ProfileData, request_profile};

    #[tokio::test]
    async fn test_request_profile() -> Result<(), reqwest::Error> {
        let user: ProfileData = request_profile("widici").await?;
        assert_eq!(&user.joined, "2021-05-14T20:14:08Z");
        Ok(())
    }
}

