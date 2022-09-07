mod github;

use dotenv::dotenv;
use github::{GitHub, GitHubErr};
use inquire::{InquireError, Select, Text};
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
    #[error("GitHub Error")]
    GitHubErr(#[from] GitHubErr),
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
            Actions::Search => "Search users...",
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
        let action = Select::new(
            "GitHub",
            vec![Actions::Search, Actions::UserInfo, Actions::Repos],
        )
        .prompt()?;

        match action {
            Actions::Search => {
                let query = Text::new("User Id").prompt()?;
                github.search_users(&query).await?;
            }
            Actions::UserInfo => todo!(),
            Actions::Repos => todo!(),
        }
    }
}
