use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::NjallaClient;
use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub name: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub id: String,
    pub status: String,
    pub os: String,
    pub expiry: String,
    pub autorenew: bool,
    pub ssh_key: String,
    pub ips: Vec<String>,
    pub reverse_name: String,
    pub os_state: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewServer {
    pub name: String,
    #[serde(rename = "type")]
    pub server_type: String,
    pub os: String,
    pub ssh_key: String,
    pub months: u32,
}

#[derive(Debug, Deserialize)]
struct ServersResponse {
    servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
struct ImagesResponse {
    images: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TypesResponse {
    types: Vec<String>,
}

impl NjallaClient {
    /// Lists all servers on the account.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn list_servers(&self) -> Result<Vec<Server>> {
        let resp: ServersResponse = self.call("list-servers", json!({})).await?;
        Ok(resp.servers)
    }

    /// Lists available OS images for server creation.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn list_server_images(&self) -> Result<Vec<String>> {
        let resp: ImagesResponse = self.call("list-server-images", json!({})).await?;
        Ok(resp.images)
    }

    /// Lists available server instance types.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or API rejection.
    pub async fn list_server_types(&self) -> Result<Vec<String>> {
        let resp: TypesResponse = self.call("list-server-types", json!({})).await?;
        Ok(resp.types)
    }

    /// Creates a new server. Returns the created server.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or invalid parameters.
    pub async fn add_server(&self, server: &NewServer) -> Result<Server> {
        let params = serde_json::to_value(server)?;
        self.call("add-server", params).await
    }

    /// Stops a running server. Data is preserved.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the server is not found.
    pub async fn stop_server(&self, id: &str) -> Result<Server> {
        self.call("stop-server", json!({ "id": id })).await
    }

    /// Starts a stopped server.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the server is not found.
    pub async fn start_server(&self, id: &str) -> Result<Server> {
        self.call("start-server", json!({ "id": id })).await
    }

    /// Restarts a server.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the server is not found.
    pub async fn restart_server(&self, id: &str) -> Result<Server> {
        self.call("restart-server", json!({ "id": id })).await
    }

    /// Factory resets a server with new settings. **Destroys all data.**
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or invalid parameters.
    pub async fn reset_server(
        &self,
        id: &str,
        os: &str,
        ssh_key: &str,
        server_type: &str,
    ) -> Result<Server> {
        self.call(
            "reset-server",
            json!({ "id": id, "os": os, "ssh_key": ssh_key, "type": server_type }),
        )
        .await
    }

    /// Removes a server. **Destroys all data.**
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the server is not found.
    pub async fn remove_server(&self, id: &str) -> Result<Server> {
        self.call("remove-server", json!({ "id": id })).await
    }
}
