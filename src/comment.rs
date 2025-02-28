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

    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, pr_number
    );
    println!("Attempting to post comment to: {}", url);

    let response = client
        .post(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.v3+json")
        .bearer_auth(token)
        .json(&comment)
        .send()
        .await
        .map_err(|err| {
            eprintln!("Failed to post comment. Error: {:?}", err);
            err
        })?;

    if response.status().is_success() {
        println!("Successfully posted comment to PR #{}", pr_number);
    } else {
        let error_body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
        eprintln!("Failed to post comment. Response: {}", error_body);
    }

    Ok(())
}
