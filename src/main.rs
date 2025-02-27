use anyhow::Result;
use std::env;
use fibonacci::fibonacci;

mod extract;
mod fibonacci;
mod comment;

fn main() -> Result<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    println!("Hello, world!");

    let owner = "micheal-ndoh";
    let repo = "project_fibot";
    let pr_number = 1; 
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string()) == "true";
    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string()).parse().unwrap_or(100);

    let pr_content = extract::fetch_pr_content(owner, repo, pr_number).await?;
    let numbers = extract::extract_numbers_from_pr(&pr_content);

    if enable_fib {
        let fib_numbers: Vec<u32> = numbers.iter().flat_map(|&n| fibonacci::fibonacci(n, max_threshold)).collect();
        let message = format!("Fibonacci numbers: {:?}", fib_numbers);
        comment::post_comment(owner, repo, pr_number, message).await?;
        for numbers in fib_numbers{

            println!("The fibonacci of {} is: {:?}", numbers,fibonacci(numbers, max_threshold));
         }
    }

    Ok(())
}
