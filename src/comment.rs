use reqwest::{Client, Error};
use serde::Serialize;

#[derive(Serialize)]
struct Comment {
    body: String,
}

async fn post_comment(pr_number: u32, message: String) -> Result<(), Error> {
    let client = Client::new();
    let comment = Comment { body: message };
    client
        .post(format!(
            "https://api.github.com/repos/{owner}/{repo}/issues/{}/comments",
            pr_number
        ))
        .bearer_auth("GITHUB_TOKEN")
        .json(&comment)
        .send()
        .await?;
    Ok(())
}
