use eyre::{Error, Result};
use tokio;

mod cli;
mod git;
mod gpt;
mod prompts;

// TODO: Summarize each folder flag
#[tokio::main]
async fn main() -> Result<(), Error> {
    cli::Cli::run().await?;

    Ok(())
}
