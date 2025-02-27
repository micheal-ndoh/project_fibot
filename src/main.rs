// main.rs
use anyhow::Result;
use extract::{extract_numbers_from_pr, fetch_pr_content};
use fibonacci::fibonacci;
use comment::post_comment;

mod fibonacci;
mod comment;
mod extract;


const MAX_FIBONACCI_THRESHOLD: u32 = 100;

#[tokio::main]
async fn main() -> Result<()> {
   
    let owner = "micheal-ndoh";
    let repo = "project_fibot";
    let pr_number = 1;
    let content = fetch_pr_content(owner, repo, pr_number).await?;

    let numbers = extract_numbers_from_pr(&content);

    let results: Vec<(u32, Vec<u32>)> = numbers
        .into_iter()
        .map(|num| (num, fibonacci(num, MAX_FIBONACCI_THRESHOLD)))
        .collect();

    for (num, fib) in results {
       
        let comment = format!("The Fibonacci sequence for {} is {:?}", num, fib);
        post_comment(owner, repo, pr_number, comment).await?;
    }

    Ok(())
}