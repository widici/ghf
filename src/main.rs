mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let data: RepoData = request_repos("widici").await?;
    let field = data.stars;

    if field.is_some() {
        println!("{}", field.unwrap())
    } else if field.is_some() {
        println!("{:#?}", field)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::profile::{ProfileData, request_profile};

    #[tokio::test]
    async fn test_request_profile() -> Result<(), reqwest::Error> {
        let _: ProfileData = request_profile("widici").await?;
        Ok(())
    }
}

