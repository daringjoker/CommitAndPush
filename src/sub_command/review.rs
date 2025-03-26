use std::process::Command;

use super::SubCommand;
use crate::{ai::AI, args::Arguments, config::Config};

pub struct Review {}

impl SubCommand for Review {
    async fn run(
        &self,
        _args: Arguments,
        _config: Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let diffcommand = Command::new("git")
            .args(["diff", "--patch", "origin/dev", "HEAD"])
            .output()
            .expect("failed to execute process");

        let diff = String::from_utf8(diffcommand.stdout).unwrap();

        let ai = AI::new(diff.into());

        let review_prompt = "You are an expert senior software engineer with over 15 years of experience in software development, architecture, and code review. Your task is to perform a detailed, critical, and thorough review of a pull request based on a Git diff that I will provide. The code is written by a senior software engineer, and your review should reflect the high standards expected of such a contributor. Your review should include the following:
Code Quality: Assess the readability, maintainability, and structure of the code. Check for adherence to common coding standards (e.g., naming conventions, consistent formatting, modular design). Highlight any areas where the code could be more concise, elegant, or easier to understand.

Correctness: Verify that the changes achieve their intended purpose as implied by the diff. Look for potential bugs, edge cases, or logical errors that might have been overlooked. Be skeptical and question assumptions made in the code.

Performance: Evaluate the efficiency of the code. Identify any potential performance bottlenecks, unnecessary resource usage, or areas where optimization could be applied (e.g., time complexity, memory usage, I/O operations).

Security: Check for security vulnerabilities such as injection risks, improper input validation, exposure of sensitive data, or misuse of libraries/APIs. Suggest mitigations if issues are found.

Testing: Assess whether the changes include sufficient test coverage (e.g., unit tests, integration tests). If tests are missing or inadequate, point this out and suggest what should be added. If tests are present, evaluate their quality and effectiveness.

Design and Architecture: Critique the design decisions made in the diff. Are the changes consistent with good software design principles (e.g., SOLID, DRY)? Do they align with the broader system architecture? Suggest improvements if the design could be more robust or scalable.

Documentation: Review the inline comments, function/class documentation, and any updated README or external docs. Ensure they are clear, accurate, and sufficient for future maintainers to understand the intent and usage of the code.

Risks and Trade-offs: Highlight any risks introduced by the changes (e.g., breaking changes, backward compatibility issues, technical debt). Discuss trade-offs made in the implementation and whether they are justified.

Suggestions: Provide actionable feedback for improvement. Where applicable, suggest alternative approaches, refactorings, or best practices that could enhance the code. Be specific and constructive.

Tone: Be professional but firm. Your review should be critical and hold the code to a high standard, as expected from a senior engineer’s work. Avoid vague praise or overly gentle feedback—focus on substance and precision.
I will provide the Git diff below. Please analyze it line-by-line and provide your review in a structured format, such as:
Summary: A brief overview of your impression of the changes.

Detailed Feedback: Line-by-line or section-by-section critique with specific observations and recommendations, numbered for clarity.

Conclusion: Final thoughts and whether you would approve, request changes, or reject the pull request, with justification.

Please proceed with the review once the diff is provided.";

        let result = ai.query(review_prompt.into()).await;

        if result.is_empty() {
            println!("No response from AI");
            return Ok(());
        }

        println!("{}", result[0]);

        Ok(())
    }
}
