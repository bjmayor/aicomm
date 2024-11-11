use crate::{AiService, Message};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaAdapter {
    pub host: String,
    pub model: String,
    pub client: Client,
}

#[derive(Serialize)]
pub struct OllamaChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OllamaChatCompletionResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaMessage,
    pub done: bool,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    pub prompt_eval_duration: u64,
    pub eval_count: u32,
    pub eval_duration: u64,
}

impl OllamaAdapter {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            model: model.into(),
            client: Client::new(),
        }
    }

    pub fn new_local(model: impl Into<String>) -> Self {
        Self::new("http://localhost:11434", model)
    }
}

impl Default for OllamaAdapter {
    fn default() -> Self {
        Self::new_local("llama3.2")
    }
}

impl AiService for OllamaAdapter {
    async fn complete(&self, messages: &[Message]) -> Result<String> {
        let request = OllamaChatCompletionRequest {
            model: self.model.clone(),
            stream: false,
            messages: messages.iter().map(|m| m.into()).collect(),
        };

        let response = self
            .client
            .post(format!("{}/api/chat", self.host))
            .json(&request)
            .send()
            .await?;
        let response: OllamaChatCompletionResponse = response.json().await?;

        Ok(response.message.content)
    }
}

impl From<Message> for OllamaMessage {
    fn from(message: Message) -> Self {
        Self {
            role: message.role.to_string(),
            content: message.content,
        }
    }
}

impl From<&Message> for OllamaMessage {
    fn from(message: &Message) -> Self {
        Self {
            role: message.role.to_string(),
            content: message.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Role;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn ollama_complete_should_work() {
        let adapter = OllamaAdapter::new_local("llama3.2");
        let response = adapter
            .complete(&[Message {
                role: Role::User,
                content: "Hello, world!".to_string(),
            }])
            .await;
        println!("{:?}", response);
    }
}
