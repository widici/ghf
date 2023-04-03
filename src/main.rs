mod api;
use crate::api::profile::{ProfileData, request_profile};
use crate::api::repo::{RepoData, request_repos};
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
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_data: UserData = UserData::new("jake").await?;

    for (name, value) in FieldsIter::new(&user_data.profile_data)
        .chain(FieldsIter::new(&user_data.repo_data))
    {
        if let Some(value) = value.downcast_ref::<Option<String>>() {
            if let Some(inner) = value.as_ref().filter(|v| !v.is_empty()) {
                println!("{}: {}", name, inner);
            }
        } else if let Some(value) = value.downcast_ref::<Option<i32>>() {
            if let Some(inner) = value.as_ref() {
                println!("{}: {}", name, inner);
            }
        } else {
            continue
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{UserData};

    #[tokio::test]
    async fn requests_works() -> Result<(), reqwest::Error> {
        let _: UserData = UserData::new("widici").await?;
        Ok(())
    }
}