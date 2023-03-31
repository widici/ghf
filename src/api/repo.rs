use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    #[serde(default)]
    pub language: Option<String>,
    #[serde(rename = "forks_count")]
    pub forks: Option<i32>,
    #[serde(rename = "stargazers_count")]
    pub stars: Option<i32>,
}

pub async fn request_repos(username: &str) -> Result<Vec<RepoData>, reqwest::Error> {
    let repos: Vec<RepoData> = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}/repos", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await?
        .json()
        .await?;

    return Ok(repos)
}
