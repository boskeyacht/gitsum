use crate::git::Git;
use clap::{Args, Parser, Subcommand};
use eyre::Error;
use reqwest::Client;

#[derive(Debug, Parser)]
#[command(
    author,
    version = "0.1",
    about = "Summarize a github repository",
    long_about = "
`gitsum` is a tool for summarizing github repositories using gpt. `gitsum` allows you to 
summarize an entire repository (useful in cases where there is no README), folders, or files."
)]
pub struct Cli {
    /// The command to run
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Summarize a github repository
    #[command(name = "sum")]
    Sum(SumArgs),
}

#[derive(Debug, Args)]
pub struct SumArgs {
    /// The username of the repository owner
    #[clap(short, long)]
    pub username: String,

    /// The name of the repository
    #[clap(short, long)]
    pub repo: String,

    /// The branch of the repository
    #[clap(short, long)]
    pub branch: String,

    /// Your github api key
    #[clap(short, long)]
    pub git_key: Option<String>,

    /// Your openai api key
    #[clap(short, long)]
    pub open_ai_key: Option<String>,

    /// The folder to sumamrize
    #[clap(short, long)]
    pub folder: Option<String>,

    /// The file to save the summaries to
    #[clap(short = 's', long)]
    pub file: Option<String>,

    /// The maximum number of tokens to generate in the chat completion.
    #[clap(short, long, default_value = "4096")]
    pub max_tokens: Option<i64>,

    /// What sampling temperature to use, between 0 and 2
    #[clap(short = 'x', long, default_value = "0.7")]
    pub temperature: Option<f64>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    ///
    /// The model considers the results of the tokens with top_p probability mass.
    /// It it recommended to alter this or temperature but not both.
    #[clap(short, long, default_value = "1.0")]
    pub top_p: Option<f64>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far.
    #[clap(short, long, default_value = "0.0")]
    pub presence_penalty: Option<f64>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far.
    #[clap(short = 'q', long, default_value = "0.0")]
    pub frequency_penalty: Option<f64>,
}

impl Cli {
    pub async fn run() -> Result<(), Error> {
        let args = Self::parse();

        match args.command {
            Commands::Sum(args) => {
                let git_key = match args.git_key {
                    Some(key) => {
                        if key.is_empty() {
                            match std::env::var("GITHUB_KEY").ok() {
                                Some(key) => key,
                                None => {
                                    eprintln!("GITHUB_KEY environment variable not set");

                                    std::process::exit(1);
                                }
                            }
                        } else {
                            key
                        }
                    }

                    None => match std::env::var("GITHUB_KEY").ok() {
                        Some(key) => key,
                        None => {
                            eprintln!("GITHUB_KEY environment variable not set");

                            std::process::exit(1);
                        }
                    },
                };

                let open_ai_key = match std::env::var("OPEN_AI_KEY").ok() {
                    Some(key) => {
                        if key.is_empty() {
                            match std::env::var("OPEN_AI_KEY").ok() {
                                Some(key) => key,
                                None => {
                                    eprintln!("OPEN_AI_KEY environment variable not set");

                                    std::process::exit(1);
                                }
                            }
                        } else {
                            key
                        }
                    }

                    None => {
                        eprintln!("OPEN_AI_KEY environment variable not set");

                        std::process::exit(1);
                    }
                };

                let mut git = Git::new(
                    &git_key,
                    &open_ai_key,
                    &args.username,
                    &args.repo,
                    &args.branch,
                );

                git.get_contents(Client::new()).await?;

                if args.file.is_some() {
                    if args.folder.is_some() {
                        git.summarize_file(&args.folder.unwrap(), &args.file.unwrap())
                            .await?;
                    } else {
                        return Err(eyre::eyre!("You must specify a folder to summarize a file"));
                    }
                } else if args.folder.is_some() {
                    git.summarize_folder(&args.folder.unwrap()).await?;
                } else {
                    git.summarize_repository().await?;
                }
            }
        };

        Ok(())
    }
}
