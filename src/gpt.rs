use eyre::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: i64,

    pub completion_tokens: i64,

    pub total_tokens: i64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: String,

    pub content: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ChatChoice {
    pub index: i64,

    pub message: ChatMessage,

    pub finish_reason: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,

    pub object: String,

    pub created: i64,

    pub choices: Vec<ChatChoice>,

    pub usage: ChatUsage,
}

#[derive(Debug, Default, Serialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,

    pub model: String,

    pub max_tokens: i64,

    pub temperature: f64,

    pub top_p: f64,

    pub frequency_penalty: f64,

    pub presence_penalty: f64,
}

impl ChatRequest {
    pub fn new(
        message: String,
        model: String,
        max_tokens: i64,
        temperature: f64,
        top_p: f64,
        frequency_penalty: f64,
        presence_penalty: f64,
    ) -> Self {
        Self {
            messages: vec![ChatMessage {
                role: String::from("user"),
                content: message,
            }],
            model,
            max_tokens,
            temperature,
            top_p,
            frequency_penalty,
            presence_penalty,
        }
    }

    pub async fn send(&self, open_ai_key: &str, client: Client) -> Result<ChatResponse, Error> {
        let url = "https://api.openai.com/v1/chat/completions";

        let res = client
            .post(url)
            .json(self)
            .bearer_auth(open_ai_key)
            .send()
            .await;

        let res = res?.json::<ChatResponse>().await?;

        Ok(res)
    }
}

#[derive(Debug)]
pub struct ChatRequestBuilder {
    message: String,

    model: String,

    max_tokens: i64,

    temperature: f64,

    top_p: f64,

    frequency_penalty: f64,

    presence_penalty: f64,

    stop: Vec<String>,
}

impl ChatRequestBuilder {
    pub fn new(
        message: String,
        model: String,
        max_tokens: i64,
        temperature: f64,
        top_p: f64,
        frequency_penalty: f64,
        presence_penalty: f64,
    ) -> Self {
        Self {
            message,
            model,
            max_tokens,
            temperature,
            top_p,
            frequency_penalty,
            presence_penalty,
            stop: vec![],
        }
    }

    pub fn messages(mut self, message: String) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = frequency_penalty;
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = stop;
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest::new(
            self.message,
            self.model,
            self.max_tokens,
            self.temperature,
            self.top_p,
            self.frequency_penalty,
            self.presence_penalty,
        )
    }
}

impl Default for ChatRequestBuilder {
    fn default() -> Self {
        Self {
            message: String::new(),
            model: String::from("gpt-3.5-turbo"),
            max_tokens: 0,
            temperature: 0.0,
            top_p: 0.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stop: vec![String::from("")],
        }
    }
}
