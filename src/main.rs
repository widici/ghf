mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let data: RepoData = request_repos("widici").await?;
    let field = data.language;
    println!("{}", field.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::profile::{ProfileData, request_profile};

    #[tokio::test]
    async fn request_profile_works() -> Result<(), reqwest::Error> {
        let _: ProfileData = request_profile("widici").await?;
        Ok(())
    }
}

