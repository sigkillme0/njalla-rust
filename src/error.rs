use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct JsonRpcError {
    pub(crate) code: i64,
    pub(crate) message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("api ({code}): {message}")]
    Api { code: i64, message: String },

    #[error("missing 'result' in response")]
    MissingResult,

    #[error("env: {0}")]
    Env(#[from] std::env::VarError),

    #[error("not found: {0}")]
    NotFound(String),
}

impl From<JsonRpcError> for Error {
    fn from(e: JsonRpcError) -> Self {
        Self::Api {
            code: e.code,
            message: e.message,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
