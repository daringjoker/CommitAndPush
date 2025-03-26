use core::panic;
use openai_api_rs::v1::{
    api::OpenAIClient,
    chat_completion::{self, ChatCompletionRequest},
    common::GPT4_O,
};
use std::env;

#[derive(Debug, Clone)]
pub struct AI {
    api_key: String,
    context_data: String,
    model_id: String,
}

impl AI {
    pub fn new(context: Option<String>) -> AI {
        let maybe_api_key = env::var("OPENAI_API_KEY");

        if maybe_api_key.is_err() {
            println!(
                "Error reading Api Key from Environment: {:?}",
                maybe_api_key.err()
            );
            panic!("OPENAI_API_KEY not set in the Environment");
        }
        let api_key = maybe_api_key.unwrap();

        return AI {
            api_key,
            model_id: GPT4_O.to_string(),
            context_data: context.unwrap_or("".to_string()),
        };
    }

    pub async fn query(&self, prompt: String) -> Vec<String> {
        let complete_prompt = if self.context_data.len() > 0 {
            format!("{}\n\n{}", prompt, self.context_data,)
        } else {
            prompt
        };

        let client = OpenAIClient::builder().with_api_key(&self.api_key).build();
        if client.is_err() {
            println!("Error creating OpenAI client: {:?}", client.err());
            panic!("Failed to create OpenAI client");
        }

        let mut client = client.unwrap();
        let req = ChatCompletionRequest::new(
            self.model_id.to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text(complete_prompt.to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        );

        let result = client.chat_completion(req).await;

        if result.is_err() {
            println!("Error: {:?}", result.err());
            return vec![];
        }

        if result.as_ref().unwrap().choices.len() == 0 {
            return vec![];
        }

        return result
            .unwrap()
            .choices
            .iter()
            .filter(|choice| choice.message.content.is_some())
            .map(|choice| choice.message.content.clone().unwrap())
            .collect();
    }
}
