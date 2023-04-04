use crate::git::Git;
use clap::{Args, Parser, Subcommand};
use eyre::Error;
use reqwest::Client;
use spinners::{Spinner, Spinners};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Summarize a github repository
    #[command(name = "sum")]
    Sum(SumArgs),
}

#[derive(Debug, Parser)]
#[command(author, version = "0.1", about = "Summarize a github repository", long_about = None)]
pub struct Cli {
    /// The command to run
    #[clap(subcommand)]
    pub command: Commands,
}
impl Cli {
    pub async fn run() -> Result<(), Error> {
        let args = Self::parse();

        match args.command {
            Commands::Sum(args) => {
                let mut spinner = Spinner::new(
                    Spinners::Dots9,
                    format!("Summarizing {}/{}\n\n", args.username, args.repo),
                );

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

                let open_ai_key = match std::env::var("OPENAI_KEY").ok() {
                    Some(key) => {
                        if key.is_empty() {
                            match std::env::var("OPENAI_KEY").ok() {
                                Some(key) => key,
                                None => {
                                    eprintln!("OPENAI_KEY environment variable not set");

                                    std::process::exit(1);
                                }
                            }
                        } else {
                            key
                        }
                    }

                    None => {
                        eprintln!("OPENAI_KEY environment variable not set");

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

                spinner.stop();

                git.get_contents(Client::new()).await?;
                git.summarize_repository().await?;
            }
        };

        Ok(())
    }
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
}
