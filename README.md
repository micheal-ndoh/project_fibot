
# `PROJECT FIBOT`

**Description**

An action in GitHub written in Rust that scans the text of the pull request for numbers, calculates its Fibonacci number, and proceeds to post a comment with the results. The action has two parameters(e.g., a flag to enable Fibonacci calculation and a threshold limit).

## **Objectives / Milestiones / Key points**

> Here a the objectives and milestones that are covered in this projects

### Step 1. Printing and confirming from logs

- A Rust program that prints “Hello, world!” when executed.

  - A GitHub workflow that runs the “Hello, world!” program and outputs in the action logs.

  - A minimum working GitHub Action written in Rust that is running on GitHub correctly.

### Step 2: Workflows

- An action.yml workflow file that implements input parsing both on the rust side and on the action side and accepts parameters `enable_fib` and `max_threshold`.

### Step 3: Fibonacci function

- A rust basic level function with sample edge cases to test invoke the fibonacci function.

- Will have the main parts of extracting numbers and calling the fibonacci function embedded within the core logic.

### Step 4: Github API

- Integrated functionality that extracts numbers and computes their Fibonacci values.
- A rust code that posts a comment to a pull request GitHub API, which integrates authentication and the computed results.

### Step 5: Verifying and testing

- The action should be able to process the pull request, compute ,fibonacci numbers  for extracted values and post a comment with the results
- A fully functioning GitHub Action tested in a dummy repository, with successful execution on pull requests.

### Step 6: Display

- A comprehensive README with setup instructions, parameter configurations, and code comments; plus any necessary code refinements.
- A polished, documented, and fully working GitHub Action that is ready for public use.

---

## Usage

1. The workflow runs on pull request so you will have to forked the repository and create a new branch then make some changes by adding numbers or a sentence containing numbers which will be extracted and the fibonacci frequency will be display on the comment section of the pull request

2. Inputs
   To use this action you should forked the repository and edit the fibonacci `.github/workflows/fibbot.yml` and the two parameters the `enable_fib` which enables the fibonacci calculation and the `max_threshold` which is the maximum number the fibonacci action can/should perform.

| Input Name      | Description                                       | Default Value |
|---------------|----------------------------------------------------|-----------|
| `enable_fib`  | Enables Fibonacci calculation (`true` or `false`). | `true`    |
| `max_threshold` | Maximum Fibonacci number that can be computed.    | `100`   |

### 3. How It Works

1. The action scans the PR for added or modified file content.
2. It extracts all numerical values from the PR content.
3. It computes the Fibonacci value for each number up to the `max_threshold`.
4. It posts a comment on the PR with the extracted numbers and their Fibonacci values.

### 4. Example Comment Output

When a PR contains numbers, FibBot will post a comment like:

```
Extracted numbers: [5, 8, 12]
Fibonacci calculation results: [5, 21, 144]
```

- Special Thanks all my Tutor [MARC JAZZ](https://github.com/Marcjazz) for briefing us on want the project is expect to do so we don't succeed with a wrong task completed.
