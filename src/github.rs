use std::fmt::Display;

use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client, StatusCode,
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
    #[error("Error response")]
    ErrResponse(String),
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
            .await?;

        let status = res.status();
        let body = res.text().await?;
        if !status.is_success() {
            Err(GitHubErr::ErrResponse(body))
        } else {
            let user = serde_json::from_str(&body)?;
            Ok(user)
        }
    }

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
        write!(f, "👋 {}({}) \n", self.name, self.login)?;
        if let Some(bio) = &self.bio {
            write!(f, "❔ {} \n", bio)?;
        }

        if let Some(com) = &self.company {
            write!(f, "🖥️  @ {} \n", com)?;
        }

        if let Some(loc) = &self.location {
            write!(f, "📍 {} \n", loc)?;
        }

        if let Some(email) = &self.email {
            write!(f, "📧 {} \n", email)?;
        }

        write!(
            f,
            "Public repos 📔 {}, public gists 📕 {} \n",
            self.public_repos, self.public_gists
        )?;

        write!(
            f,
            "followers 🏃 {}, following ❤️ {} \n",
            self.followers, self.following
        )?;

        write!(f, "More at 📘 {} \n", self.html_url)
    }
}
