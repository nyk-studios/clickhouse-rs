mod entities;
use anyhow::{bail, Ok, Result};
use entities::QueryResult;

pub struct Client<'a> {
    server_url: &'a str,
}
impl<'client> Client<'client> {
    /// Creates a new [`Client`].
    /// server_url: The url of the server to connect to. It could contains user and password if needs. Example: http://user:password@localhost:8123
    pub fn new(server_url: impl Into<&'client str>) -> Self {
        Self {
            server_url: server_url.into(),
        }
    }

    pub async fn ping(&self) -> Result<bool> {
        let response = reqwest::get(format!("{}/ping", self.server_url)).await?;

        let text = response.text().await?;

        Ok(text == "Ok.\n")
    }

    pub async fn execute(&self, query: impl Into<String>) -> Result<()> {
        let response = reqwest::Client::new()
            .post(self.server_url)
            .body(query.into())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text().await?);
        }

        Ok(())
    }

    pub async fn query<TResult>(&self, query: impl Into<String>) -> Result<QueryResult<TResult>>
    where
        TResult: serde::de::DeserializeOwned,
    {
        let query = query.into();
        if query.is_empty() {
            bail!("Query is empty");
        }
        if !query.contains("FORMAT JSON") {
            bail!("Query must contains FORMAT JSON");
        }

        let response = reqwest::Client::new()
            .post(self.server_url)
            .body(query)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text().await?);
        }

        Ok(response.json::<QueryResult<TResult>>().await?)
    }
}
