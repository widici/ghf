use std::collections::HashMap;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    pub language: Option<String>,
    #[serde(rename = "forks_count")]
    pub forks: Option<i32>,
    #[serde(rename = "stargazers_count")]
    pub stars: Option<i32>,
}

pub async fn request_repos(username: &str) -> Result<RepoData, reqwest::Error> {
    let repos: Vec<RepoData> = reqwest::Client::new()
        .get(format!("https://api.github.com/users/{}/repos", username))
        .header(USER_AGENT, "ghfetch")
        .send()
        .await?
        .json()
        .await?;

    let (sum_stars, sum_forks) = repos.iter().fold((0, 0), |acc, i| {
        (acc.0 + i.stars.unwrap(), acc.1 + i.forks.unwrap())
    });

    let mut languages: Vec<String> = Vec::new();
    for repo in repos.iter() {
        if repo.language.is_some() && !languages.contains(&repo.language.as_ref().unwrap()) {
            languages.push(repo.language.as_ref().unwrap().to_string())
        }
    }
    let languages: String = languages.join(", ");

    return Ok( RepoData { language: Some(languages), forks: Some(sum_forks), stars: Some(sum_stars) } )
}