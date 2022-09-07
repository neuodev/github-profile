use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION},
    Client,
};
use thiserror::Error;

const GITHUB_API: &str = "https://api.github.com";

#[derive(Debug, Error)]
pub enum GitHubErr {
    #[error("Unable to set the request header")]
    HeaderErr(#[from] InvalidHeaderValue),
    #[error("API error")]
    APIErr(#[from] reqwest::Error),
}

pub struct GitHub<'a> {
    token: &'a str,
}

impl<'a> GitHub<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
    pub async fn search_users(&self, q: &str) -> Result<(), GitHubErr> {
        let url = format!("{GITHUB_API}/search/users");
        let client = Client::new();
        let res = client
            .get(url)
            .headers(self.get_common_headers()?)
            .send()
            .await?
            .text()
            .await?;

        println!("{:?}", res);

        Ok(())
    }
    pub fn user() {}
    pub fn repos() {}

    fn get_common_headers(&self) -> Result<HeaderMap, GitHubErr> {
        let mut headers = HeaderMap::new();
        headers.append(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );

        headers.append(AUTHORIZATION, HeaderValue::from_str(self.token)?);

        Ok(headers)
    }
}
