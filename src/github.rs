use std::fmt::Display;

use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::Deserialize;
use thiserror::Error;

const GITHUB_API: &str = "https://api.github.com";

#[derive(Debug, Error)]
pub enum GitHubErr {
    #[error("Unable to set the request header")]
    HeaderErr(#[from] InvalidHeaderValue),
    #[error("API error")]
    APIErr(#[from] reqwest::Error),
    #[error("Serialization error")]
    SerdeErr(#[from] serde_json::Error),
}

pub struct GitHub<'a> {
    token: &'a str,
    username: &'a str,
}

impl<'a> GitHub<'a> {
    pub fn new(token: &'a str, username: &'a str) -> Self {
        Self { token, username }
    }
    pub async fn search_users(&self, q: &str) -> Result<SearchRes, GitHubErr> {
        let url = format!("{GITHUB_API}/search/users");
        let client = Client::new();
        let res = client
            .get(url)
            .headers(self.get_common_headers()?)
            .query(&[("q", q)])
            .send()
            .await?
            .text()
            .await?;
        let users: SearchRes = serde_json::from_str(&res)?;
        Ok(users)
    }
    pub async fn user(&self, username: &str) -> Result<User, GitHubErr> {
        let url = format!("{GITHUB_API}/users/{username}");

        let client = Client::new();
        let res = client
            .get(url)
            .headers(self.get_common_headers()?)
            .send()
            .await?
            .text()
            .await?;

        let user = serde_json::from_str(&res)?;
        Ok(user)
    }
    pub fn repos() {}

    fn get_common_headers(&self) -> Result<HeaderMap, GitHubErr> {
        let mut headers = HeaderMap::new();
        headers.append(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );

        headers.append(AUTHORIZATION, HeaderValue::from_str(self.token)?);
        headers.append(USER_AGENT, HeaderValue::from_str(self.username)?);

        Ok(headers)
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchEntry {
    pub login: String,
    id: u32,
    html_url: String,
    #[serde(rename = "type")]
    profile_type: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchRes {
    pub total_count: u32,
    pub items: Vec<SearchEntry>,
}

impl Display for SearchEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})  #{} {}",
            self.login, self.profile_type, self.id, self.html_url,
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
    html_url: String,
    name: String,
    bio: Option<String>,
    company: Option<String>,
    location: Option<String>,
    email: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) 👋", self.name, self.login)?;
        if let Some(bio) = &self.bio {
            write!(f, "❔ {}", bio)?;
        }

        if let Some(com) = &self.company {
            write!(f, "work ⛑️ @ {}", com)?;
        }

        if let Some(loc) = &self.location {
            write!(f, "📍 {}", loc)?;
        }

        if let Some(email) = &self.email {
            write!(f, "📧 {}", email)?;
        }

        write!(
            f,
            "Public repos 😃{}, public gists 🍞 {}",
            self.public_repos, self.public_gists
        )?;

        write!(
            f,
            "followers 🏃 {}, following ❤️‍🔥 {} ",
            self.followers, self.following
        )?;

        write!(f, "More at 📘 {}", self.html_url)
    }
}
