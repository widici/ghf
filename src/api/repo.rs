use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::api::request::request;

#[derive(Serialize, Deserialize, fields_iter::FieldsInspect, Debug)]
pub struct RepoData {
    #[serde(rename = "forks_count")]
    pub forks: i32,
    #[serde(rename = "stargazers_count")]
    pub stars: i32,
    #[serde(rename = "language")]
    pub languages: Option<String>,
}

pub async fn request_repos(username: &str) -> Result<RepoData> {
    let url = &format!("https://api.github.com/users/{}/repos", username);
    let repos: Vec<RepoData> = request(url)
        .await?
        .json()
        .await?;

    let (sum_stars, sum_forks) = repos.iter().fold((0, 0), |acc, i| {
        (acc.0 + i.stars, acc.1 + i.forks)
    });

    let mut languages_vec: Vec<String> = Vec::new();
    for repo in repos.iter() {
        if repo.languages.is_some() && !languages_vec.contains(&repo.languages.as_ref().unwrap()) {
            languages_vec.push(repo.languages.as_ref().unwrap().to_string())
        }
    }
    let languages: Option<String> = Some(languages_vec.join(", "));

    return Ok( RepoData { languages, forks: sum_forks, stars: sum_stars } )
}