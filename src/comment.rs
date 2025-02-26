use reqwest::{Client, Error};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Comment {
    body: String,
}

async fn post_comment(pr_number: u32, message: String) -> Result<(), Error> {
    let owner = "micheal-ndoh";
    let repo = "project_fibot";
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