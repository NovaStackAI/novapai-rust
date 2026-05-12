// NovaPAI Rust SDK Example
// Cargo.toml:
//   [dependencies]
//   async-openai = "0.28"
//   tokio = { version = "1", features = ["full"] }
//   futures = "0.3"
// Docs: https://api.novapai.ai

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let config = OpenAIConfig::new()
        .with_api_key("your-api-key")
        .with_api_base("https://api.novapai.ai/router/v1");

    let client = Client::with_config(config);

    basic_chat(&client).await;
    stream_chat(&client).await;
}

// ── Basic Chat ──────────────────────────────────────────────
async fn basic_chat(client: &Client<OpenAIConfig>) {
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-v4-pro")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a helpful assistant.")
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("Hello!")
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let response = client.chat().create(request).await.unwrap();
    println!("{}", response.choices[0].message.content.as_deref().unwrap_or(""));
}

// ── Streaming ───────────────────────────────────────────────
async fn stream_chat(client: &Client<OpenAIConfig>) {
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-v4-pro")
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("Tell me a joke")
            .build()
            .unwrap()])
        .build()
        .unwrap();

    let mut stream = client.chat().create_stream(request).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                for choice in &response.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                    }
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
    println!();
}
