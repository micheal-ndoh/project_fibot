use std::env;
use fibonacci::fibonacci;
use comment::post_comment;
use extract::extract_numbers_from_pr;
use anyhow::{Result, Context};
use octocrab::Octocrab;

mod extract;
mod fibonacci;
mod comment;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    let owner = env::var("GITHUB_REPOSITORY")
        .context("GITHUB_REPOSITORY environment variable is not set")?;
    let parts: Vec<&str> = owner.split('/').collect();
    let owner = parts[0];
    let repo = parts[1];

    let pr_number: u32 = match env::var("GITHUB_REF") {
        Ok(ref_value) => {
            ref_value
                .split('/')
                .nth(2)
                .and_then(|s| s.parse().ok())
                .context("Failed to parse PR number")?
        }
        Err(_) => {
            println!("GITHUB_REF environment variable is not set. Defaulting to PR number 0.");
            0
        }
    };

    println!("GitHub PR details: owner={}, repo={}, PR#={}", owner, repo, pr_number);

    let octocrab = Octocrab::builder()
        .personal_token(env::var("GITHUB_TOKEN").context("GITHUB_TOKEN environment variable is not set")?)
        .build()?;
    let enable_fib = env::var("INPUT_ENABLE_FIB")
        .unwrap_or_else(|_| "true".to_string()) == "true";
    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "1000000".to_string())
        .parse()
        .unwrap_or(1000000);

    let pr_content = octocrab
        .pulls(owner, repo)
        .get(pr_number.into())
        .await
        .context("Failed to fetch PR content")?;
    let pr_body = pr_content.body.unwrap_or_default();

    let numbers = extract::extract_numbers_from_pr(&pr_body);

    println!("Extracted numbers: {:?}", numbers);

    if enable_fib {
        let fib_results: Vec<u32> = numbers
            .iter()
            .map(|&n| fibonacci(n, max_threshold))
            .collect();

        let message = format!(
            "Extracted numbers: {:?}\nFibonacci calculation results: {:?}",
            numbers, fib_results
        );

        comment::post_comment(owner, repo, pr_number, message)
            .await
            .context("Failed to post comment")?;

        println!("Fibonacci calculation results: {:?}", fib_results);
    }

    Ok(())
}

