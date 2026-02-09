use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::NjallaClient;
use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: u32,
    #[serde(rename = "prio", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: String,
    pub ttl: u32,
    #[serde(rename = "prio", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct RecordsResponse {
    records: Vec<Record>,
}

impl NjallaClient {
    /// Lists all DNS records for a domain.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn list_records(&self, domain: &str) -> Result<Vec<Record>> {
        let resp: RecordsResponse = self
            .call("list-records", json!({ "domain": domain }))
            .await?;
        Ok(resp.records)
    }

    /// Adds a DNS record to a domain. Returns the created record with its ID.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the record is invalid.
    pub async fn add_record(&self, domain: &str, record: &NewRecord) -> Result<Record> {
        let mut params = serde_json::to_value(record)?;
        params["domain"] = json!(domain);
        self.call("add-record", params).await
    }

    /// Edits an existing DNS record. All fields are sent; fetch first if patching.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the record is not found.
    pub async fn edit_record(&self, domain: &str, record: &Record) -> Result<()> {
        let mut params = serde_json::to_value(record)?;
        params["domain"] = json!(domain);
        self.call_void("edit-record", params).await
    }

    /// Removes a DNS record by ID from a domain.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the record is not found.
    pub async fn remove_record(&self, domain: &str, id: &str) -> Result<()> {
        self.call_void("remove-record", json!({ "domain": domain, "id": id }))
            .await
    }
}
