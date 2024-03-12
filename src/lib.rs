mod entities;
use anyhow::{bail, Ok, Result};
use entities::QueryResult;

#[derive(Debug)]
pub struct Client {
    server_url: String,
}

impl Client {
    /// Creates a new [`Client`].
    /// server_url: The url of the server to connect to. It could contains user and password if needs. Example: http://user:password@localhost:8123
    pub fn new(server_url: String) -> Self {
        Self { server_url }
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
    pub fn execute(&self, query: &str) -> Result<()> {
        let mut retries = 0;

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        loop {
            retries += 1;
            let response = reqwest::blocking::Client::new()
                .post(self.server_url.clone())
                .headers(HeaderMap::from_iter(vec![(
                    header::CONTENT_ENCODING,
                    "gzip".parse().unwrap(),
                )]))
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
    pub async fn execute(&self, query: &str) -> Result<()> {
        use std::io::Write;

        use flate2::Compression;
        use reqwest::header::{self, HeaderMap};

        let mut retries = 0;

        let query: String = query.into();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        loop {
            retries += 1;
            let response = reqwest::Client::new()
                .post(self.server_url.clone())
                .headers(HeaderMap::from_iter(vec![(
                    header::CONTENT_ENCODING,
                    "gzip".parse().unwrap(),
                )]))
                .body(body.clone())
                .send()
                .await?;

            if !response.status().is_success() {
                if retries <= 3 {
                    println!("Error: {}", response.text().await?);
                    println!("Retrying {}/3", retries);
                    continue;
                } else {
                    bail!("Error: {}\nQuery: {}", response.text().await?, query);
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn query<TResult>(&self, query: &str) -> Result<QueryResult<TResult>>
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
            .post(self.server_url.clone())
            .headers(HeaderMap::from_iter(vec![(
                header::CONTENT_ENCODING,
                "gzip".parse().unwrap(),
            )]))
            .body(body)
            .send()?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text()?);
        }

        Ok(response.json::<QueryResult<TResult>>()?)
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn query<TResult>(&self, query: &str) -> Result<QueryResult<TResult>>
    where
        TResult: serde::de::DeserializeOwned,
    {
        use flate2::Compression;
        use reqwest::header::{self, HeaderMap};
        use std::io::Write;

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
            .post(self.server_url.clone())
            .headers(HeaderMap::from_iter(vec![(
                header::CONTENT_ENCODING,
                "gzip".parse().unwrap(),
            )]))
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Error: {}", response.text().await?);
        }

        Ok(response.json::<QueryResult<TResult>>().await?)
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn send(&self, query: &str) -> Result<reqwest::Response> {
        use flate2::Compression;
        use reqwest::header::{self, HeaderMap};
        use std::io::Write;
        use tracing::debug;

        debug!("Query: {}", query);

        if query.is_empty() {
            bail!("Query is empty");
        }
        if !query.contains("FORMAT JSON") {
            bail!("Query must contains FORMAT JSON");
        }

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        reqwest::Client::new()
            .post(self.server_url.clone())
            .headers(HeaderMap::from_iter(vec![(
                header::CONTENT_ENCODING,
                "gzip".parse().unwrap(),
            )]))
            .body(body)
            .send()
            .await
            .map_err(anyhow::Error::from)
    }

    #[cfg(feature = "blocking")]
    pub async fn send(&self, query: &str) -> Result<reqwest::Response> {
        use flate2::Compression;
        use reqwest::header::{self, HeaderMap};
        use std::io::Write;
        use tracing::debug;

        debug!("Query: {}", query);

        if query.is_empty() {
            bail!("Query is empty");
        }
        if !query.contains("FORMAT JSON") {
            bail!("Query must contains FORMAT JSON");
        }

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(query.as_bytes())?;
        let body = encoder.finish()?;

        reqwest::blocking::Client::new()
            .post(self.server_url.clone())
            .headers(HeaderMap::from_iter(vec![(
                header::CONTENT_ENCODING,
                "gzip".parse().unwrap(),
            )]))
            .body(body)
            .send()
            .map_err(anyhow::Error::from)
    }
}
