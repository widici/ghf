use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, fields_iter::FieldsInspect, Debug)]
pub struct ProfileData {
    pub name: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "blog")]
    pub site: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    #[serde(rename = "twitter_username")]
    pub twitter: Option<String>,
    #[serde(rename = "public_repos")]
    pub repos: Option<i32>,
    #[serde(rename = "public_gists")]
    pub gists: Option<i32>,
    pub followers: Option<i32>,
    #[serde(rename = "created_at")]
    pub joined: Option<String>,
    pub id: Option<i32>,
}

pub async fn request_profile(username: &str) -> Result<ProfileData, reqwest::Error> {
    let result: ProfileData = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}