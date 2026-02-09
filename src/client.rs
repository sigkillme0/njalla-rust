use reqwest::Client;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::time::Duration;

use crate::error::{Error, JsonRpcError, Result};

const ENDPOINT: &str = "https://njal.la/api/1/";
const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'static str,
    id: u32,
    method: &'a str,
    params: Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    result: Option<Value>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Clone)]
pub struct NjallaClient {
    auth: String,
    http: Client,
}

impl NjallaClient {
    /// Creates a new client with the given API token.
    ///
    /// # Errors
    ///
    /// Returns `Error::Http` if the HTTP client fails to initialize.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        Ok(Self {
            auth: format!("Njalla {}", token.into()),
            http: Client::builder().timeout(TIMEOUT).build()?,
        })
    }

    /// Creates a client from the `NJALLA_API_TOKEN` environment variable.
    ///
    /// # Errors
    ///
    /// Returns `Error::Env` if `NJALLA_API_TOKEN` is not set, or
    /// `Error::Http` if the HTTP client fails to initialize.
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Self::new(std::env::var("NJALLA_API_TOKEN")?)
    }

    pub(crate) async fn call<T: DeserializeOwned>(&self, method: &str, params: Value) -> Result<T> {
        let body = JsonRpcRequest {
            jsonrpc: "2.0",
            id: 1,
            method,
            params,
        };

        let resp = self
            .http
            .post(ENDPOINT)
            .header("Authorization", &self.auth)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<JsonRpcResponse>()
            .await?;

        if let Some(err) = resp.error {
            return Err(err.into());
        }

        Ok(serde_json::from_value(
            resp.result.ok_or(Error::MissingResult)?,
        )?)
    }

    pub(crate) async fn call_void(&self, method: &str, params: Value) -> Result<()> {
        let _: Value = self.call(method, params).await?;
        Ok(())
    }
}
