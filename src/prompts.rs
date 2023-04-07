use crate::gpt::ChatRequestBuilder;
use eyre::{eyre, Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

pub const REPOSITORY_SUMMARY_PROMPT: &str =
    "Thoroughly summarize this github repository given summaries of its folders: {{summaries}}. 
Make sure to consider every file in the repository. Return a JSON object for your answer. 
Make sure your entire answer is in the JSON object! Use the below schema for your answer.
{
    \"summary\": \"\"
}";

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositorySummaryPrompt {
    pub prompt: String,
}

impl RepositorySummaryPrompt {
    pub fn new(contents: &str) -> RepositorySummaryPrompt {
        RepositorySummaryPrompt {
            prompt: REPOSITORY_SUMMARY_PROMPT.replace("{{summaries}}", contents),
        }
    }

    pub fn replace_attributes<T: ToString>(&mut self, attributes: Vec<(T, T)>) {
        let mut prompt = self.prompt.clone();

        for (key, value) in attributes {
            prompt = prompt.replace(&key.to_string(), &value.to_string());
        }

        self.prompt = prompt;
    }

    pub async fn send(&self, key: &str) -> Result<RepositorySummaryResponse, Error> {
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

        let summary =
            from_str::<RepositorySummaryResponse>(&res.choices[0].message.content.clone());

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

impl Default for RepositorySummaryPrompt {
    fn default() -> Self {
        RepositorySummaryPrompt {
            prompt: REPOSITORY_SUMMARY_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositorySummaryResponse {
    pub summary: String,
}

pub const FOLDER_WIDE_SUMMARY_PROMPT: &str =
    "Thoroughly summarize this folder given summaries of the files inside it: {{files}}. 
Make sure to consider every file in the repository. Return a JSON object for your answer. 
Make sure your entire answer is in the JSON object! Use the below schema for your answer.
{
    \"summary\": \"\"
}";

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderWideSummaryPrompt {
    pub prompt: String,
}

impl FolderWideSummaryPrompt {
    pub fn new(contents: &str) -> FolderWideSummaryPrompt {
        FolderWideSummaryPrompt {
            prompt: FOLDER_WIDE_SUMMARY_PROMPT.replace("{{files}}", contents),
        }
    }

    pub fn replace_attributes<T: ToString>(&mut self, attributes: Vec<(T, T)>) {
        let mut prompt = self.prompt.clone();

        for (key, value) in attributes {
            prompt = prompt.replace(&key.to_string(), &value.to_string());
        }

        self.prompt = prompt;
    }

    pub async fn send(&self, key: &str) -> Result<FolderWideSummaryResponse, Error> {
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

        let summary =
            from_str::<FolderWideSummaryResponse>(&res.choices[0].message.content.clone());

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

impl Default for FolderWideSummaryPrompt {
    fn default() -> Self {
        FolderWideSummaryPrompt {
            prompt: FOLDER_WIDE_SUMMARY_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderWideSummaryResponse {
    pub summary: String,
}

pub const FILE_SUMMARY_PROMPT: &str =
    "Thoroughly summarize this code file given the contents: {{file}}. 
Make sure to consider every file in the repository. Return a JSON object for your answer. 
Make sure your entire answer is in the JSON object! Use the below schema for your answer.
{
    \"summary\": \"\"
}";

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSummaryPrompt {
    pub prompt: String,
}

impl FileSummaryPrompt {
    pub fn new(contents: &str) -> FileSummaryPrompt {
        FileSummaryPrompt {
            prompt: FILE_SUMMARY_PROMPT.replace("{{file}}", contents),
        }
    }

    pub fn replace_attributes<T: ToString>(&mut self, attributes: Vec<(T, T)>) {
        let mut prompt = self.prompt.clone();

        for (key, value) in attributes {
            prompt = prompt.replace(&key.to_string(), &value.to_string());
        }

        self.prompt = prompt;
    }

    pub async fn send(&self, key: &str) -> Result<FileSummaryResponse, Error> {
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

        let summary = from_str::<FileSummaryResponse>(&res.choices[0].message.content.clone());

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

impl Default for FileSummaryPrompt {
    fn default() -> Self {
        FileSummaryPrompt {
            prompt: FILE_SUMMARY_PROMPT.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSummaryResponse {
    pub summary: String,
}
