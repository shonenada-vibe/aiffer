use serde::{Deserialize, Serialize};

/// Configuration for the AI API connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
}

/// Error type for AI operations.
#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API returned error: {0}")]
    Api(String),
    #[error("Missing API configuration: {0}")]
    Config(String),
}

/// The system prompt used to instruct the AI.
const SYSTEM_PROMPT: &str = r#"You are a code review assistant. The user has been reviewing a git diff and has left comments on specific lines of code. Your job is to:

1. Understand what changes the user is requesting based on their review comments
2. Summarize the requested changes clearly and concisely
3. For each comment, explain what specific code change should be made
4. If comments suggest conflicting changes, point that out
5. Provide a prioritized list of changes to make

Format your response in clear markdown with sections for each file that needs changes. Be specific about what code to change and how."#;

/// OpenAI-compatible chat message.
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI-compatible chat completion request.
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

/// OpenAI-compatible chat completion response.
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    content: String,
}

/// Send a review payload to an OpenAI-compatible API and return the response.
pub async fn submit_review(
    config: &AiConfig,
    review_text: &str,
) -> Result<String, AiError> {
    if config.endpoint.is_empty() {
        return Err(AiError::Config("API endpoint URL is required".to_string()));
    }
    if config.api_key.is_empty() {
        return Err(AiError::Config("API key is required".to_string()));
    }
    if config.model.is_empty() {
        return Err(AiError::Config("Model name is required".to_string()));
    }

    let url = format!(
        "{}/chat/completions",
        config.endpoint.trim_end_matches('/')
    );

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: review_text.to_string(),
            },
        ],
        temperature: 0.3,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AiError::Api(format!(
            "HTTP {} — {}",
            status,
            if body.len() > 500 { &body[..500] } else { &body }
        )));
    }

    let chat_response: ChatResponse = response.json().await?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| AiError::Api("No response choices returned".to_string()))
}
