const GITHUB_API: &str = "https://api.github.com";

pub struct GitHub<'a> {
    token: &'a str,
}

impl<'a> GitHub<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
    pub fn search(&self) {}
    pub fn user() {}
    pub fn repos() {}
}
