use crate::gpt::ChatRequestBuilder;
use eyre::{eyre, Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

pub const SUMMARY_PROMPT: &str = "Thoroughly summarize this github repository given the directories and files containing code: {{contents}}. 
Make sure to consider every file in the repository. Return a JSON object for your answer. 
Make sure your entire answer is in the JSON object! Use the below schema for your answer.
{
    \"summary\": \"\"
}";

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryPrompt {
    pub prompt: String,
}

impl SummaryPrompt {
    pub fn new(contents: &str) -> SummaryPrompt {
        SummaryPrompt {
            prompt: SUMMARY_PROMPT.replace("{{contents}}", contents),
        }
    }

    pub fn replace_attributes<T: ToString>(&mut self, attributes: Vec<(T, T)>) {
        let mut prompt = self.prompt.clone();

        for (key, value) in attributes {
            prompt = prompt.replace(&key.to_string(), &value.to_string());
        }

        self.prompt = prompt;
    }

    pub async fn send(&self, key: &str) -> Result<SummaryResponse, Error> {
        let res = ChatRequestBuilder::default()
            .messages(self.prompt.clone())
            .temperature(0.7)
            .max_tokens(2000)
            .top_p(1.0)
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .send(&key, Client::new())
            .await?;

        let summary = from_str::<SummaryResponse>(&res.choices[0].message.content.clone());

        match summary {
            Ok(summary) => Ok(summary),
            Err(e) => Err(eyre!(
                "failed to sent gpt request: {}: {}",
                &res.choices[0].message.content.clone(),
                e
            )),
        }
    }
}

impl Default for SummaryPrompt {
    fn default() -> Self {
        SummaryPrompt {
            prompt: SUMMARY_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryResponse {
    pub summary: String,
}
