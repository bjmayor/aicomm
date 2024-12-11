use anyhow::Result;
use chat_core::{Chat, Message};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    PgPool,
};
use std::collections::HashSet;
use swiftide::{
    integrations::{self},
    query::{
        self, answers,
        query_transformers::{self},
        response_transformers,
    },
    traits::{EmbeddingModel, SimplePrompt},
};
use swiftide_pgvector::PgVector;
use tracing::info;

use crate::{AppConfig, VECTOR_SIZE};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum AppEvent {
    NewChat(Chat),
    AddToChat(Chat),
    RemoveFromChat(Chat),
    NewMessage(Message),
}

#[derive(Debug)]
struct Notification {
    // users being impacted, so we should send the notification to them
    user_id: i64,
    event: Message,
}

// pg_notify('chat_message_created', row_to_json(NEW)::text);
#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageCreated {
    message: Message,
    members: HashSet<i64>,
}

pub async fn setup_pg_listener(config: &AppConfig) -> anyhow::Result<()> {
    let db_url = &config.server.db_url;
    let mut listener = PgListener::connect(db_url).await?;
    listener.listen("chat_message_created").await?;
    info!("Listening to chat_message_created");

    let pool = PgPoolOptions::new().connect(db_url).await?;
    // let fastembed = integrations::fastembed::FastEmbed::try_default()?;
    // let client = integrations::ollama::Ollama::default()
    //     .with_default_prompt_model("llama3.2")
    //     .to_owned();
    let client = integrations::openai::OpenAI::builder()
        .default_embed_model("text-embedding-3-small")
        .default_prompt_model("gpt-4o-mini")
        .build()?;

    let mut stream = listener.into_stream();

    let pool_clone = pool.clone();
    while let Some(Ok(notif)) = stream.next().await {
        info!("Received notification: {:?}", notif);
        let bots = get_bots(&pool).await?;
        if let Some(notification) = Notification::load(notif.channel(), notif.payload(), &bots) {
            let pool = pool_clone.clone();
            let client = client.clone();
            tokio::spawn(async move {
                if let Err(e) = notification.process(&pool, client.clone(), client).await {
                    tracing::error!("Failed to process notification: {:?}", e);
                }
            });
        }
    }

    Ok(())
}

impl Notification {
    pub(crate) fn load(r#type: &str, payload: &str, bots: &HashSet<i64>) -> Option<Self> {
        match r#type {
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(payload).ok()?;
                let mut members: HashSet<_> = payload.members;
                members.remove(&payload.message.sender_id);
                if members.len() == 1 {
                    let bot_id = members.iter().next().copied();
                    if let Some(bot_id) = bot_id {
                        if bots.contains(&bot_id) {
                            return Some(Self {
                                user_id: bot_id,
                                event: payload.message,
                            });
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    async fn process(
        &self,
        pool: &PgPool,
        client: impl SimplePrompt + Clone + 'static,
        embed: impl EmbeddingModel + Clone + 'static,
    ) -> anyhow::Result<()> {
        let store = PgVector::try_new(pool.clone(), VECTOR_SIZE as _).await?;
        let pipeline = query::Pipeline::default()
            .then_transform_query(query_transformers::GenerateSubquestions::from_client(
                client.clone(),
            ))
            .then_transform_query(query_transformers::Embed::from_client(embed.clone()))
            .then_retrieve(store)
            .then_transform_response(response_transformers::Summary::from_client(client.clone()))
            .then_answer(answers::Simple::from_client(client));
        info!("Processing notification: {:?}", self.event.id);
        let answer = pipeline.query(&self.event.content).await?;
        info!("Got answer, Writing to db...");

        let _: (i64,) = sqlx::query_as(
            r#"
          INSERT INTO messages (chat_id, sender_id, content)
          VALUES ($1, $2, $3)
          RETURNING id
          "#,
        )
        .bind(self.event.chat_id as i64)
        .bind(self.user_id)
        .bind(answer.answer().to_string())
        .fetch_one(pool)
        .await?;
        Ok(())
    }
}

async fn get_bots(pool: &PgPool) -> Result<HashSet<i64>> {
    let bots: Vec<(i64,)> = sqlx::query_as(r#"SELECT id FROM users WHERE is_bot"#)
        .fetch_all(pool)
        .await?;
    Ok(bots.into_iter().map(|b| b.0).collect())
}
