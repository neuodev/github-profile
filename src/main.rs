mod github;

use dotenv::dotenv;
use github::GitHub;
use inquire::{InquireError, Select};
use std::{
    env::{self, VarError},
    fmt::Display,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppErr {
    #[error("Missing github auth token")]
    AuthTokenErr(#[from] VarError),
    #[error("Failed to get user input")]
    InputErr(#[from] InquireError),
}
#[derive(Debug)]
enum Actions {
    Search,
    UserInfo,
    Repos,
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Actions::Search => "Search...",
            Actions::UserInfo => "User profile",
            Actions::Repos => "Inspect Repos",
        };

        write!(f, "{}", txt)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    dotenv().ok();
    let auth_token = env::var("GITHUB_TOKEN")?;
    let github = GitHub::new(&auth_token);

    loop {
        let input = Select::new(
            "GitHub",
            vec![Actions::Search, Actions::UserInfo, Actions::Repos],
        )
        .prompt()?;
    }

    Ok(())
}
