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

    #[cfg(feature = "blocking")]
    pub fn ping(&self) -> Result<bool> {
        let response = reqwest::blocking::get(format!("{}/ping", self.server_url))?;

        let text = response.text()?;

        Ok(text == "Ok.\n")
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn ping(&self) -> Result<bool> {
        let response = reqwest::get(format!("{}/ping", self.server_url)).await?;

        let text = response.text().await?;

        Ok(text == "Ok.\n")
    }

    #[cfg(feature = "blocking")]
    pub fn execute(&self, query: impl Into<String>) -> Result<()> {
        let mut retries = 0;

        let query: String = query.into();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        loop {
            retries += 1;
            let response = reqwest::blocking::Client::new()
                .post(self.server_url)
                .body(body.clone())
                .send()?;

            if !response.status().is_success() {
                if retries <= 3 {
                    println!("Error: {}", response.text().unwrap());
                    println!("Retrying {}/3", retries);
                    continue;
                } else {
                    bail!("Error: {}", response.text()?);
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn execute(&self, query: impl Into<String>) -> Result<()> {
        use std::io::Write;

        use flate2::Compression;

        let mut retries = 0;

        let query: String = query.into();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        loop {
            retries += 1;
            let response = reqwest::Client::new()
                .post(self.server_url)
                .body(body.clone())
                .send()
                .await?;

            if !response.status().is_success() {
                if retries <= 3 {
                    println!("Error: {}", response.text().await?);
                    println!("Retrying {}/3", retries);
                    continue;
                } else {
                    bail!("Error: {}", response.text().await?);
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn query<TResult>(&self, query: impl Into<String>) -> Result<QueryResult<TResult>>
    where
        TResult: serde::de::DeserializeOwned,
    {
        use flate2::Compression;

        let query = query.into();
        if query.is_empty() {
            bail!("Query is empty");
        }
        if !query.contains("FORMAT JSON") {
            bail!("Query must contains FORMAT JSON");
        }

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        let response = reqwest::blocking::Client::new()
            .post(self.server_url)
            .body(body)
            .send()?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text()?);
        }

        Ok(response.json::<QueryResult<TResult>>()?)
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn query<TResult>(&self, query: impl Into<String>) -> Result<QueryResult<TResult>>
    where
        TResult: serde::de::DeserializeOwned,
    {
        use flate2::Compression;
        use std::io::Write;

        let query = query.into();
        if query.is_empty() {
            bail!("Query is empty");
        }
        if !query.contains("FORMAT JSON") {
            bail!("Query must contains FORMAT JSON");
        }

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        let response = reqwest::Client::new()
            .post(self.server_url)
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text().await?);
        }

        Ok(response.json::<QueryResult<TResult>>().await?)
    }
}
