use reqwest::{Client, Error};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct PullRequest {
    body: String,
}

pub async fn fetch_pr_content(owner: &str, repo: &str, pr_number: u32) -> Result<String, Error> {
    let owner = "micheal-ndoh";
    let repo = "project_fibot";
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_number
    );
    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<PullRequest>()
        .await?;
    Ok(response.body)
}

pub fn extract_numbers_from_pr(content: &str) -> Vec<u32> {
    content
        .replace("-", " ")
        .split_whitespace()
        .map(|word| word.chars().filter(|c| !c.is_ascii_punctuation()).collect::<String>())
        .filter_map(|word| word.parse().ok())
        .collect()
}