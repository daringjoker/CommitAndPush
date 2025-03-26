use std::process::Command;

use crate::ai::AI;
use crate::args::Arguments;
use crate::config::Config;
use crate::sub_command::SubCommand;

pub struct PullRequest {}
impl SubCommand for PullRequest {
    async fn run(
        &self,
        _args: Arguments,
        config: Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let diffcommand = Command::new("git")
            .args(["diff", "--patch", "origin/dev", "HEAD"])
            .output()
            .expect("failed to execute process");

        let diff = String::from_utf8(diffcommand.stdout).unwrap();

        let ai = AI::new(diff.into());

        let pr_description_prompt = if config.has_categories() {
            let category_prompt_details = config.get_category_determination_prompt();
            let category_prompt = format!(
                "What category does this PR belong to?\n{}reply with the category name only",
                category_prompt_details
            );

            let result = ai.query(category_prompt).await;

            if result.is_empty() {
                println!("No response from AI");
                return Ok(());
            }

            let pr_category = result[0].trim();
            config.get_prompt_for_category(pr_category)
        } else {
            config.default_prompt.clone()
        };

        // println!("PR Description Prompt: {}", pr_description_prompt);

        let result = ai.query(pr_description_prompt).await;

        if result.is_empty() {
            println!("No response from AI");
            return Ok(());
        }

        println!("{}", result[0]);

        Ok(())
    }
}
