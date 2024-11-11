use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{AiService, Message, Role};

pub struct OpenAIAdapter {
    pub host: String,
    pub api_key: String,
    pub model: String,
    pub client: Client,
}

#[derive(Serialize)]
pub struct OpenAIChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAIChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<OpenAIChoice>,
    pub usage: OpenAIUsage,
}

#[derive(Deserialize)]
pub struct OpenAIChoice {
    pub index: i32,
    pub message: OpenAIMessage,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
    pub completion_tokens_details: OpenAICompletionTokensDetails,
}

#[derive(Deserialize)]
pub struct OpenAICompletionTokensDetails {
    pub reasoning_tokens: i32,
    pub accepted_prediction_tokens: i32,
    pub rejected_prediction_tokens: i32,
}

impl OpenAIAdapter {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: "https://api.openai.com/v1".to_string(),
            api_key: api_key.into(),
            model: model.into(),
            client: Client::new(),
        }
    }
}

impl AiService for OpenAIAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = OpenAIChatCompletionRequest {
            model: self.model.clone(),
            messages: messages.iter().map(|m| m.into()).collect(),
        };
        let url = format!("{}/chat/completions", self.host);
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        let response_text = response.text().await?;
        println!("{}", response_text);

        let mut completion: OpenAIChatCompletionResponse =
            serde_json::from_str(&response_text).unwrap();

        let content = completion
            .choices
            .pop()
            .ok_or(anyhow::anyhow!("No response"))?
            .message
            .content;
        Ok(content)
    }
}

impl From<Message> for OpenAIMessage {
    fn from(message: Message) -> Self {
        OpenAIMessage {
            role: message.role.to_string(),
            content: message.content,
        }
    }
}

impl From<&Message> for OpenAIMessage {
    fn from(messages: &Message) -> Self {
        OpenAIMessage {
            role: messages.role.to_string(),
            content: messages.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_complete() {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let adapter = OpenAIAdapter::new(api_key, "gpt-3.5-turbo");
        let response = adapter
            .complete(&[Message {
                role: Role::User,
                content: "Hello, world!".to_string(),
            }])
            .await;
        println!("{:?}", response);
        // assert!(response.is_ok());
    }
}
