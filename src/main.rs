use anyhow::{Result, Context};
use serde::de::value;
use std::env;
use extract::{extract_numbers_from_pr, fetch_pr_content};
use fibonacci::fibonacci;
use comment::post_comment;

mod extract;
mod fibonacci;
mod comment;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");

    let owner_repo = env::var("GITHUB_REPOSITORY")
        .context("GITHUB_REPOSITORY environment variable is not set")?;
    let parts: Vec<&str> = owner_repo.split('/').collect();
    let owner = parts[0];
    let repo = parts[1];

    let pr_number: u32 = env::var("PR_NUMBER")
    .context("PR_NUMBER environment variable is not set")?
    .parse()
    .context("Failed to parse PR number")?;


    println!("GitHub PR details: owner={}, repo={}, PR#={}", owner, repo, pr_number);

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string()) == "true";
    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);

    let pr_content = fetch_pr_content(owner, repo, pr_number)
        .await
        .context("Failed to fetch PR file diffs")?;
    
    let numbers = extract_numbers_from_pr(&pr_content);
    println!("Extracted numbers from PR files: {:?}", numbers);

    if enable_fib {
        let fib_results: Vec<u128> = numbers
            .iter()
            .map(|&n| fibonacci(n))
            .collect();

        let message = format!(
            "Extracted numbers: {:?}\nFibonacci results: {:?}",
            numbers, fib_results
        );

        post_comment(owner, repo, pr_number, message)
            .await
            .context("Failed to post comment")?;
        println!("Fibonacci calculation results: {:?}", fib_results);

    }

    Ok(())
}
