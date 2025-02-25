use reqwest::{Client, Error};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Comment {
    body: String,
}

pub async fn post_comment(owner: &str, repo: &str, pr_number: u32, message: String) -> Result<(), Error> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let client = Client::new();
    let comment = Comment { body: message };
    client
        .post(&format!(
            "https://api.github.com/repos/{}/{}/issues/{}/comments",
            owner, repo, pr_number
        ))
        .bearer_auth(token)
        .json(&comment)
        .send()
        .await?;
    Ok(())
}