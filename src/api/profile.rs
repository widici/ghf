use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::api::request::request;

#[derive(Serialize, Deserialize, fields_iter::FieldsInspect, Debug)]
pub struct ProfileData {
    pub login: Option<String>,
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
    pub joined: Option<String>,
    pub id: i32,
}

pub async fn request_profile(username: &str) -> Result<ProfileData> {
    let result: ProfileData = request(&format!("https://api.github.com/users/{}", username))
        .await?
        .json()
        .await?;

    Ok(result)
}