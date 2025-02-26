[![Run FibBot Action](https://github.com/micheal-ndoh/project_fibot/actions/workflows/publish.yml/badge.svg)](https://github.com/micheal-ndoh/project_fibot/actions/workflows/publish.yml)

# `PROJECT FIBOT`
2 4 2

**Description**

 A GitHub Action in Rust that scans pull request content for numbers, calculates their Fibonacci numbers, and posts a comment with the results. The action supports two parameters (e.g., a flag to enable Fibonacci calculation and a threshold limit).

## **Objectives / Milestiones / Key points**

> Here a the objectives and milestones that are covered in this projects

### Step 1: Printing and confirming from logs

- A Rust program that prints "Hello, world!" when executed.
  - A GitHub workflow that run the "Hello, world!" program and output in the action logs.
  - A minimal working GitHub Action written in Rust that is successfully running on GitHub.

### Step 2: Workflows

- A action.yml workflow file that accepts two parameters `enable_fib` and `max_threshold` and also implements input parsing in the rust code.
  - A demonstration (via logs or test cases) that the parameters are correctly used and validated
  - A action that correctly recieves and processes inputs from the Github workflow

### Step 3: Fibonacci function

- A function that parses a sample string (from the pull request content) and extracts numerical values
- A fibonacci function in rust with tests covering edge cases and efficiency
- Should have core logic of both number extraction and fibonacci calculation withh test cases that confirms their functionalities

### Step 4: Github API

- Integrated functionality that extracts numbers and computes their Fibonacci values.
- A rust code that posts a comment to a pull request GitHub API, which integrates authentication and the computed results.

### Step 5: Verifying and testing

- The action should be able to process the pull request, compute ,fibonacci numbers  for extracted values and post a comment with the results
- A fully functioning GitHub Action tested in a dummy repository, with successful execution on pull requests.

### Step 6: Display

- A comprehensive README with setup instructions, parameter configurations, and code comments; plus any necessary code refinements.
- A polished, documented, and fully working GitHub Action that is ready for public use.
