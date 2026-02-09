use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::NjallaClient;
use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
    pub status: String,
    pub expiry: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailforwarding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nameservers: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketDomain {
    pub name: String,
    pub status: String,
    pub price: i64,
}

#[derive(Debug, Deserialize)]
struct DomainsResponse {
    domains: Vec<Domain>,
}

#[derive(Debug, Deserialize)]
struct MarketDomainsResponse {
    domains: Vec<MarketDomain>,
}

#[derive(Debug, Deserialize)]
struct TaskResponse {
    task: String,
}

#[derive(Debug, Deserialize)]
struct TaskStatus {
    status: String,
}

impl NjallaClient {
    /// Lists all domains on the account.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn list_domains(&self) -> Result<Vec<Domain>> {
        let resp: DomainsResponse = self.call("list-domains", json!({})).await?;
        Ok(resp.domains)
    }

    /// Gets detailed info for a single domain.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the domain is not found.
    pub async fn get_domain(&self, domain: &str) -> Result<Domain> {
        self.call("get-domain", json!({ "domain": domain })).await
    }

    /// Searches for available domains matching a query.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn find_domains(&self, query: &str) -> Result<Vec<MarketDomain>> {
        let resp: MarketDomainsResponse =
            self.call("find-domains", json!({ "query": query })).await?;
        Ok(resp.domains)
    }

    /// Checks the status of an async task (e.g. domain registration).
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn check_task(&self, id: &str) -> Result<String> {
        let resp: TaskStatus = self.call("check-task", json!({ "id": id })).await?;
        Ok(resp.status)
    }

    /// Registers a domain for a given number of years. Returns a task ID.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if registration is rejected.
    pub async fn register_domain(&self, domain: &str, years: u32) -> Result<String> {
        let resp: TaskResponse = self
            .call(
                "register-domain",
                json!({ "domain": domain, "years": years }),
            )
            .await?;
        Ok(resp.task)
    }
}
