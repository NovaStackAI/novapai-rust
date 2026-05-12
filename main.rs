// NovaPAI Rust SDK Example
// Cargo.toml:
//   [dependencies]
//   async-openai = "0.28"
//   tokio = { version = "1", features = ["full"] }
//   futures = "0.3"
//   serde_json = "1"
// Docs: https://novapai.ai

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateChatCompletionResponse, FunctionCall, Role,
    },
    Client,
};
use futures::StreamExt;
use serde_json::json;

#[tokio::main]
async fn main() {
    let config = OpenAIConfig::new()
        .with_api_key("your-api-key")
        .with_api_base("https://api.novapai.ai/router/v1");

    let client = Client::with_config(config);

    basic_chat(&client).await;
    stream_chat(&client).await;
    function_calling(&client).await;
    json_mode(&client).await;
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

// ── Function Calling ────────────────────────────────────────
async fn function_calling(client: &Client<OpenAIConfig>) {
    use async_openai::types::{
        ChatCompletionRequestToolArgs, FunctionDefinition,
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-v4-pro")
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content("What's the weather in Tokyo?")
            .build()
            .unwrap()])
        .tools([ChatCompletionRequestToolArgs::default()
            .function(
                FunctionDefinition::default()
                    .name("get_weather")
                    .description("Get current weather for a city")
                    .parameters(serde_json::from_value(json!({
                        "type": "object",
                        "properties": {
                            "city": { "type": "string", "description": "City name" }
                        },
                        "required": ["city"]
                    }))
                    .unwrap())
                    .build()
            )
            .build()])
        .build()
        .unwrap();

    let response = client.chat().create(request).await.unwrap();
    let tc = &response.choices[0].message.tool_calls.as_ref().unwrap()[0];
    println!("Function: {}", tc.function.name);
    println!("Args: {}", tc.function.arguments);
}

// ── JSON Mode ───────────────────────────────────────────────
async fn json_mode(client: &Client<OpenAIConfig>) {
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-v4-pro")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("Extract company info as JSON.")
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("Apple Inc. is based in Cupertino, founded in 1976.")
                .build()
                .unwrap(),
        ])
        .response_format(async_openai::types::ChatCompletionResponseFormat::JSONObject)
        .build()
        .unwrap();

    let response = client.chat().create(request).await.unwrap();
    let data: serde_json::Value = serde_json::from_str(
        response.choices[0].message.content.as_deref().unwrap_or("{}")
    ).unwrap();
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
