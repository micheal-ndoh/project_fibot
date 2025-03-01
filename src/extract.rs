use anyhow::{Result, Context};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct PullRequest {
    body: Option<String>,
}

#[derive(Deserialize)]
struct DiffEntry {
    filename: String,
    patch: Option<String>,
}

pub async fn fetch_pr_content(owner: &str, repo: &str, pr_number: u32) -> Result<String> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable is not set")?;
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/files",
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
    println!("PR Files fetched successfully");

    let files: Vec<DiffEntry> = serde_json::from_str(&response_text).context("Failed to parse PR files")?;
    let content = files.iter()
        .filter_map(|file| file.patch.as_ref())
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");
    
    Ok(content)
}

pub fn extract_numbers_from_pr(content: &str) -> Vec<u32> {
    content
        .replace("-", " ")
        .split_whitespace()
        .map(|word| word.chars().filter(|c| c.is_numeric()).collect::<String>())
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}
