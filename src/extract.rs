use anyhow::{Result, Context};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct PullRequest {
    body: Option<String>,
}

pub async fn fetch_pr_content(owner: &str, repo: &str, pr_number: u32) -> Result<String> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable is not set")?;
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_number
    );
    
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .bearer_auth(token)
        .send()
        .await
        .context("Failed to send request to GitHub API")?;

    let response_text = response.text().await.context("Failed to read response text")?;
    println!("PR Content fetched successfully");

    let pull_request: PullRequest = serde_json::from_str(&response_text).context("Failed to parse PR content")?;
    Ok(pull_request.body.unwrap_or_default())
}

pub fn extract_numbers_from_pr(content: &str) -> Vec<u32> {
    content
        .replace("-", " ")
        .split_whitespace()
        .map(|word| word.chars().filter(|c| !c.is_ascii_punctuation()).collect::<String>())
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}
