use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoData {
    language: String,
    #[serde(rename = "license.name")]
    license: String,
    #[serde(rename = "forks_count")]
    forks: i32,
    #[serde(rename = "stargazers-count")]
    stars: i32
}
