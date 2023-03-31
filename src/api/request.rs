use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
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
    pub repos: i32,
    #[serde(rename = "public_gists")]
    pub gists: i32,
    pub followers: i32,
    #[serde(rename = "created_at")]
    pub joined: String,
}

pub async fn request_user(username: &str) -> Result<UserData, reqwest::Error> {
    let result: UserData = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}