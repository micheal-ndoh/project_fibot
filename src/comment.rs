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

    // Log the URL and attempt to send the comment
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/comments",
        owner, repo, pr_number
    );
    println!("Attempting to post comment to: {}", url);

    client
        .post(&url)
        .bearer_auth(token)
        .json(&comment)
        .send()
        .await
        .map_err(|err| {
            eprintln!("Failed to post comment. Error: {:?}", err);
            err
        })?;

    println!("Successfully posted comment to PR #{}", pr_number);
    Ok(())
}
