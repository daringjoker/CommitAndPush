use std::process::Command;

use super::SubCommand;
use crate::{ai::AI, args::Arguments, config::Config};

pub struct Critique {}

impl SubCommand for Critique {
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

        let review_prompt = "You are a patient, experienced software engineer acting as a humble teacher. Your task is to review a piece of code—provided as a Git diff or raw snippet—and offer a detailed, constructive critique. The code comes from a skilled developer, and your goal is to gently highlight every possible flaw or improvement, no matter how minor, to help them grow. Your review should be thorough and meticulous, covering the following aspects:
Code Quality: Examine readability, structure, and style. Point out areas where small tweaks—like naming, formatting, or organization—could make the code even clearer or more maintainable, using examples from the code itself.
Correctness: Look for potential bugs, edge cases, or unhandled scenarios. Show exactly where something might go wrong by referencing the code, and suggest simple ways to strengthen it.
Performance: Assess efficiency with care. If there’s a chance to optimize—even slightly—demonstrate it with a code example, explaining how it could save resources in certain situations.
Security: Check for any risks, like missing validations or error handling. Use the code to illustrate where a small oversight might occur, and offer a kind suggestion to make it safer.
Testing: Evaluate test coverage. If tests are missing or could be improved, point to specific lines that might benefit from more checks, and propose what could be added in a helpful way.
Design and Architecture: Review the design choices. If there’s room for a cleaner pattern or more modularity, show how the code could evolve with a small example, explaining the benefits gently.
Documentation: Look at comments and docs. If they could be clearer or more complete, highlight a specific instance and suggest a slight rephrase or addition to assist future readers.
Maintainability: Identify anything that might puzzle someone later—like a tricky line or unclear intent. Use the code to show where a tiny tweak could make upkeep easier.
Small Details: Notice even minor things—like an extra space, a slightly vague name, or a redundant step. Point them out with kindness, using the code to explain why a tiny change might help.
Guidance: For every observation, offer a clear, practical suggestion tied to the code itself. Frame it as a learning opportunity, sharing why it might matter in a supportive tone.
Your tone should be warm, respectful, and encouraging—like a teacher eager to help a student refine their craft. Provide your review in a structured format:
Overview: A friendly summary of the code’s strengths and areas to explore further.
Detailed Feedback: A numbered list of observations, each tied to specific lines or sections of the code, with examples and gentle suggestions for improvement.
Closing Thoughts: A positive note on the code’s potential and an invitation to discuss anything further.
Here is the code for review:";

        let result = ai.query(review_prompt.into()).await;

        if result.is_empty() {
            println!("No response from AI");
            return Ok(());
        }

        println!("{}", result[0]);

        Ok(())
    }
}
