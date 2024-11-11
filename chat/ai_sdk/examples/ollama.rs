use ai_sdk::{AiService, Message, OllamaAdapter, Role};

#[tokio::main]
async fn main() {
    let adapter = OllamaAdapter::new_local("llama3.2");
    let response = adapter
        .complete(&[Message {
            role: Role::User,
            content: "世界上最长的河流是什么?".to_string(),
        }])
        .await;
    println!("{:?}", response);
}
