use crate::prompts::{
    FileSummaryPrompt, FileSummaryResponse, FolderWideSummaryPrompt, FolderWideSummaryResponse,
    RepositorySummaryPrompt,
};
use colored::*;
use eyre::{eyre, Error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tiktoken_rs::r50k_base;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct File {
    pub name: String,

    pub content: String,

    pub download_url: String,
}

impl File {
    pub fn new(name: &str, content: &str, download_url: &str) -> File {
        File {
            name: name.to_string(),
            content: content.to_string(),
            download_url: download_url.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Folder {
    pub name: String,

    pub files: HashMap<String, File>,
}

impl Folder {
    pub fn new(name: &str) -> Folder {
        Folder {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RepositoryContent {
    pub folders: HashMap<String, Folder>,

    pub readme: String,
}

impl ToString for RepositoryContent {
    fn to_string(&self) -> String {
        let mut content = String::new();

        for (name, folder) in &self.folders {
            content.push_str(&format!("## name: {}, files: {}\n", name, {
                let mut files = String::new();

                for (name, file) in &folder.files {
                    files.push_str(&format!("{}: {}\n", name, file.content));
                }

                files
            }));
        }

        content
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GitFile {
    path: String,
    #[serde(rename = "type")]
    object_type: String,
    sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitFileResponse {
    content: String,

    download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitTree {
    pub path: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitTreeResponse {
    tree: Vec<GitTree>,
}

#[derive(Debug)]
pub struct Git {
    pub git_key: String,

    pub branch: String,

    pub open_ai_key: String,

    pub repository_username: String,

    pub repository_name: String,

    pub repository_content: RepositoryContent,
}

impl Git {
    pub fn new(git_key: &str, open_ai_key: &str, username: &str, repo: &str, branch: &str) -> Git {
        Git {
            git_key: git_key.to_string(),
            open_ai_key: open_ai_key.to_string(),
            branch: branch.to_string(),
            repository_username: username.to_string(),
            repository_name: repo.to_string(),
            repository_content: RepositoryContent::default(),
        }
    }

    /// Fetches the contents of a repository given the github url,
    /// overwrites whatever is currently in the repository field.
    pub async fn get_contents(&mut self, client: Client) -> Result<(), Error> {
        if self.repository_username.is_empty() {
            return Err(eyre!("No username provided"));
        }

        if self.repository_name.is_empty() {
            return Err(eyre!("No repo name provided"));
        }

        let repo_url = format!(
            "https://api.github.com/repos/{}/{}/git/trees/{}?recursive=1",
            self.repository_username, self.repository_name, self.branch
        );

        let response = client
            .get(repo_url)
            .header("User-Agent", String::from("baribari2"))
            .header("Authorization", format!("Bearer {}", &self.git_key))
            .send()
            .await?
            .text()
            .await?;

        let tree_response: GitTreeResponse = serde_json::from_str(&response)?;

        for item in tree_response.tree {
            if item.object_type == "tree" {
                self.repository_content
                    .folders
                    .insert(item.path.clone(), Folder::new(&item.path));
            }
        }

        for (name, folder) in &mut self.repository_content.folders {
            let folder_url = format!(
                "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
                self.repository_username, self.repository_name, name, self.branch
            );

            let response = client
                .get(&folder_url)
                .header("User-Agent", String::from("baribari2"))
                .header("Authorization", format!("Bearer {}", &self.git_key))
                .send()
                .await?
                .text()
                .await?;

            let folder_contents: Vec<GitFile> = serde_json::from_str(&response)?;

            for file in folder_contents {
                if file.object_type == "file" {
                    let file_url = format!(
                        "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
                        self.repository_username, self.repository_name, file.path, self.branch
                    );

                    let response = client
                        .get(&file_url)
                        .header("User-Agent", String::from("baribari2"))
                        .header("Authorization", format!("Bearer {}", &self.git_key))
                        .send()
                        .await?
                        .text()
                        .await?;

                    let file_response: GitFileResponse = serde_json::from_str(&response)?;

                    let download_response = client
                        .get(&file_response.download_url)
                        .header("User-Agent", String::from("baribari2"))
                        .header("Authorization", format!("Bearer {}", &self.git_key))
                        .send()
                        .await?
                        .text()
                        .await?;

                    folder.files.insert(
                        file.path.clone(),
                        File::new(
                            &file.path.clone(),
                            &download_response.clone(),
                            &file_response.download_url.clone(),
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    pub async fn summarize_repository(&self) -> Result<(), Error> {
        if self.repository_username.is_empty() {
            return Err(eyre!("No username provided"));
        }

        if self.repository_name.is_empty() {
            return Err(eyre!("No repo name provided"));
        }

        if self.open_ai_key.is_empty() {
            return Err(eyre!("No key provided"));
        }

        let mut summaries = vec![String::from("")];
        for (name, _) in &self.repository_content.folders {
            let (folder_wide, _) = self.summarize_folder(name).await?;

            summaries.push(folder_wide.summary.clone());
        }

        let s = summaries
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let repo_prompt = RepositorySummaryPrompt::new(&s);

        let repo_summary = match repo_prompt.send(&self.open_ai_key).await {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                return Err(eyre!("Error: {}", e));
            }
        };

        println!(
            "{}/{} Summary: {}",
            self.repository_username, self.repository_name, repo_summary.summary
        );

        Ok(())
    }

    pub async fn summarize_folder(
        &self,
        folder: &str,
    ) -> Result<(FolderWideSummaryResponse, Vec<FileSummaryResponse>), Error> {
        if self.repository_content.folders.is_empty() {
            return Err(eyre!("No folders in specified repository"));
        }

        let mut summaries: Vec<FileSummaryResponse> = vec![];
        let bpe = r50k_base().unwrap();
        if let Some(folder) = self.repository_content.folders.get(folder) {
            for (name, file) in &folder.files {
                let file_token_size = bpe.encode_with_special_tokens(&file.content);

                if file_token_size.len() > 4096 {
                    println!("File {} is too large to summarize", name);

                    continue;
                }

                let file_summary = self.summarize_file(&folder.name, &file.name).await?;

                summaries.push(file_summary);
            }
        };

        let sum = summaries
            .iter()
            .map(|s| s.summary.clone())
            .collect::<Vec<String>>()
            .join(" ");

        let rp = FolderWideSummaryPrompt::new(&sum);

        let folder_summary_res = rp.send(&self.open_ai_key).await?;

        println!(
            "{} {}\n {}",
            folder.green().bold(),
            "summary".green().bold(),
            folder_summary_res.summary
        );

        Ok((folder_summary_res, summaries))
    }

    pub async fn summarize_file(
        &self,
        folder: &str,
        file: &str,
    ) -> Result<FileSummaryResponse, Error> {
        if self.repository_content.folders.is_empty() {
            return Err(eyre!("No folders in specified repository"));
        }

        let bpe = r50k_base().unwrap();
        let file_summary = if let Some(folder) = self.repository_content.folders.get(folder) {
            if let Some(file) = folder.files.get(file) {
                let file_token_size = bpe.encode_with_special_tokens(&file.content);

                if file_token_size.len() > 4096 {
                    return Err(eyre!("File is too large to summarize"));
                }

                let fp = FileSummaryPrompt::new(&file.content);

                let file_summary_res = fp.send(&self.open_ai_key).await?;

                println!(
                    "{} {}\n {}\n",
                    "Summary for".green().bold(),
                    file.name.green().bold(),
                    file_summary_res.summary
                );

                file_summary_res
            } else {
                return Err(eyre!("File not found"));
            }
        } else {
            return Err(eyre!("Folder not found"));
        };

        Ok(file_summary)
    }
}
