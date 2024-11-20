use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;

use crate::proxy::types::{ProxyConfig, ProxyError, ProxyResult};

/// Connection pool for managing HTTP clients
pub struct ConnectionPool {
    /// Shared client pool
    clients: Arc<Mutex<HashMap<String, Client<HttpsConnector<HttpConnector>>>>>,
    /// Configuration
    config: ProxyConfig,
}

impl ConnectionPool {
    /// Create a new connection pool with the given configuration
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Get or create a client for the given host
    pub async fn get_client(&self, host: &str) -> ProxyResult<Client<HttpsConnector<HttpConnector>>> {
        let mut clients = self.clients.lock().await;

        // Return existing client if available
        if let Some(client) = clients.get(host) {
            return Ok(client.clone());
        }

        // Create new client with HTTPS support
        let https = HttpsConnector::new();
        let client = Client::builder()
            .pool_idle_timeout(Some(self.config.timeout))
            .pool_max_idle_per_host(self.config.pool_size as u32)
            .build::<_, hyper::Body>(https);

        // Store and return the new client
        clients.insert(host.to_string(), client.clone());
        Ok(client)
    }

    /// Remove a client from the pool
    pub async fn remove_client(&self, host: &str) {
        let mut clients = self.clients.lock().await;
        clients.remove(host);
    }

    /// Clear all clients from the pool
    pub async fn clear(&self) {
        let mut clients = self.clients.lock().await;
        clients.clear();
    }

    /// Get the number of active clients
    pub async fn client_count(&self) -> usize {
        let clients = self.clients.lock().await;
        clients.len()
    }

    /// Check if a client exists for the given host
    pub async fn has_client(&self, host: &str) -> bool {
        let clients = self.clients.lock().await;
        clients.contains_key(host)
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        // Ensure we spawn a task to clean up connections
        tokio::spawn(async move {
            if let Ok(mut clients) = self.clients.try_lock() {
                clients.clear();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_connection_pool() {
        let config = ProxyConfig {
            timeout: Duration::from_secs(30),
            pool_size: 10,
            max_retries: 3,
            preserve_headers: true,
        };

        let pool = ConnectionPool::new(config);

        // Test client creation
        let client = pool.get_client("example.com").await;
        assert!(client.is_ok());

        // Test client reuse
        assert!(pool.has_client("example.com").await);
        assert_eq!(pool.client_count().await, 1);

        // Test client removal
        pool.remove_client("example.com").await;
        assert!(!pool.has_client("example.com").await);
        assert_eq!(pool.client_count().await, 0);
    }
}
