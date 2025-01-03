use crate::{AppError, AppState};
use chat_core::{AdapterType, AgentType, ChatAgent};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateAgent {
    pub name: String,
    pub r#type: AgentType,
    pub adapter: AdapterType,
    pub model: String,
    pub prompt: String,
    #[schema(value_type = Object)]
    #[serde(default = "default_args")]
    pub args: serde_json::Value,
}

fn default_args() -> serde_json::Value {
    serde_json::json!({})
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UpdateAgent {
    pub id: u64,
    #[serde(default)]
    pub prompt: String,
    #[schema(value_type = Object)]
    #[serde(default)]
    pub args: serde_json::Value,
}

#[allow(dead_code)]
impl AppState {
    /// Create a new agent for a chat
    pub async fn create_agent(
        &self,
        input: CreateAgent,
        chat_id: u64,
    ) -> Result<ChatAgent, AppError> {
        // check if agent name already exists
        if self.agent_name_exists(chat_id, input.name.clone()).await? {
            info!("agent {} already exists in chat {}", input.name, chat_id);
            return Err(AppError::CreateAgentError(format!(
                "agent {} already exists ",
                input.name
            )));
        }
        // TODO: check if model is supported by adapter
        let agent = sqlx::query_as(
            r#"
            INSERT INTO chat_agents(chat_id, name, type, adapter, model, prompt,args)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(chat_id as i64)
        .bind(input.name)
        .bind(input.r#type)
        .bind(input.adapter)
        .bind(input.model)
        .bind(input.prompt)
        .bind(input.args)
        .fetch_one(&self.pool)
        .await?;

        Ok(agent)
    }

    /// Check if an agent name exists in a chat
    pub async fn agent_name_exists(&self, chat_id: u64, name: String) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM chat_agents WHERE chat_id = $1 AND name = $2)"#,
        )
        .bind(chat_id as i64)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(exists)
    }

    /// Check if an agent id exists in a chat
    pub async fn agent_id_exists(&self, chat_id: u64, agent_id: u64) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar(
            r#"SELECT EXISTS(SELECT 1 FROM chat_agents WHERE chat_id = $1 AND id = $2)"#,
        )
        .bind(chat_id as i64)
        .bind(agent_id as i64)
        .fetch_one(&self.pool)
        .await?;
        Ok(exists)
    }

    /// List all agents for a chat
    pub async fn list_agents(&self, chat_id: u64) -> Result<Vec<ChatAgent>, AppError> {
        let agents =
            sqlx::query_as(r#"SELECT * FROM chat_agents WHERE chat_id = $1 ORDER BY id ASC"#)
                .bind(chat_id as i64)
                .fetch_all(&self.pool)
                .await?;

        Ok(agents)
    }

    /// Update an agent
    pub async fn update_agent(
        &self,
        input: UpdateAgent,
        chat_id: u64,
    ) -> Result<ChatAgent, AppError> {
        if !self.agent_id_exists(chat_id, input.id).await? {
            return Err(AppError::UpdateAgentError(format!(
                "agent {} does not exist in chat {}",
                input.id, chat_id
            )));
        }
        let prompt = input.prompt;
        let args = input.args;

        let agent = match (prompt.as_str(), &args) {
            ("", _) => {
                sqlx::query_as(r#"UPDATE chat_agents SET args = $1 WHERE chat_id = $2 AND id = $3 RETURNING *"#)
                    .bind(args)
                    .bind(chat_id as i64)
                    .bind(input.id as i64)
                    .fetch_one(&self.pool)
                    .await?
            }
            (_, _) => sqlx::query_as(
                r#"UPDATE chat_agents SET prompt = $1, args = $2 WHERE chat_id = $3 AND id = $4 RETURNING *"#,
            )
            .bind(prompt)
            .bind(args)
                .bind(chat_id as i64)
                .bind(input.id as i64)
                .fetch_one(&self.pool)
                .await?,
        };

        Ok(agent)
    }
}
#[cfg(test)]
impl CreateAgent {
    pub fn new(
        name: impl Into<String>,
        r#type: AgentType,
        adapter: AdapterType,
        model: impl Into<String>,
        prompt: impl Into<String>,
        args: impl Serialize,
    ) -> Self {
        Self {
            name: name.into(),
            r#type,
            adapter,
            model: model.into(),
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}
#[cfg(test)]
impl UpdateAgent {
    pub fn new(id: u64, prompt: impl Into<String>, args: impl Serialize) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::collections::HashMap;

    #[tokio::test]
    async fn create_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateAgent::new(
            "test",
            AgentType::Proxy,
            AdapterType::Ollama,
            "llama3.2",
            "You are a helpful assistant.".to_string(),
            HashMap::<String, String>::new(),
        );
        let agent = state
            .create_agent(input, 1)
            .await
            .expect("create agent failed");
        assert_eq!(agent.chat_id, 1);
        assert_eq!(agent.r#type, AgentType::Proxy);
        assert_eq!(agent.name, "test");
        assert_eq!(agent.adapter, AdapterType::Ollama);
        assert_eq!(agent.model, "llama3.2");
        assert_eq!(agent.prompt, "You are a helpful assistant.");
        assert_eq!(agent.args, sqlx::types::Json(serde_json::json!({})));
        Ok(())
    }

    /*
        INSERT INTO chat_agents(chat_id, name, type, prompt, args)
    VALUES (
        1,
        'translation',
        'proxy',
        'If language is Chinese, translate to English, if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content',
        '{}'
      );
         */
    #[tokio::test]
    async fn list_agents_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let agents = state.list_agents(1).await.expect("list agents failed");
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "translation");
        assert_eq!(agents[0].r#type, AgentType::Proxy);
        assert_eq!(agents[0].prompt, "If language is Chinese, translate to English, if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content");
        assert_eq!(agents[0].args, sqlx::types::Json(serde_json::json!({})));
        Ok(())
    }

    #[tokio::test]
    async fn update_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // create an agent
        let input = CreateAgent::new(
            "test".to_string(),
            AgentType::Proxy,
            AdapterType::Ollama,
            "llama3.2",
            "You are a helpful assistant.".to_string(),
            HashMap::<String, String>::new(),
        );
        let agent = state
            .create_agent(input, 1)
            .await
            .expect("create agent failed");
        // update the agent
        let input = UpdateAgent::new(
            agent.id as _,
            "Can you tell me your name?",
            HashMap::<String, String>::new(),
        );
        let agent = state
            .update_agent(input, 1)
            .await
            .expect("update agent failed");
        assert_eq!(agent.prompt, "Can you tell me your name?");
        assert_eq!(agent.args, sqlx::types::Json(serde_json::json!({})));
        Ok(())
    }
}
