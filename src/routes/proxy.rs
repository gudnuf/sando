/**
 * R4.0 Proxy Route
 * ================
 *
 * Handles proxy requests via subdomain-based routing:
 * e.g., {connection_string}.localhost:3000/path
 * Establishes holesail connections using background mode for persistent connections
 * This file is tagged for machine-readability.
 *
 * Tags: R4.1, R4.2, R4.3, R4.4, R4.5, R4.6, R4.7, R4.8, R4.9
 */
// R4.1 Dependencies
use crate::{AppState, Connection};
use axum::{
    body::Body,
    extract::{Host, OriginalUri, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::Response,
};
use lazy_static::lazy_static;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;
use tokio::process::Command as TokioCommand;

// R4.2 Background Connection Manager
// Tracks holesail background connections and their states
lazy_static! {
    static ref BACKGROUND_CONNECTIONS: Mutex<HashMap<String, BackgroundConnection>> = Mutex::new(HashMap::new());
    static ref HOLESAIL_AVAILABLE: Mutex<Option<bool>> = Mutex::new(None);
}

#[derive(Debug, Clone, serde::Serialize)]
struct BackgroundConnection {
    connection_string: String,
    port: u16,
    name: String,
    status: ConnectionStatus,
    #[serde(skip)] // Skip serializing SystemTime as it's not easily serializable
    last_used: std::time::SystemTime,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
enum ConnectionStatus {
    Starting,
    Online,
    Stopped,
    Error,
}

#[derive(Deserialize, Debug)]
struct HolesailListEntry {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Uptime")]
    uptime: String,
}

// R4.3 Subdomain-based Proxy Handler  
// Routes incoming requests from `{connection_string}.localhost:3000/*` to the appropriate backend service
#[tracing::instrument(name = "proxy_handler_subdomain", skip(app_state, body, headers))]
pub async fn proxy_handler_subdomain(
    State(app_state): State<AppState>,
    Host(host): Host,
    OriginalUri(original_uri): OriginalUri,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, StatusCode> {
    // Only handle subdomain requests, return 404 for everything else
    let connection_string = match extract_subdomain(&host, &app_state.host) {
        Ok(cs) => cs,
        Err(_) => return Err(StatusCode::NOT_FOUND), // Not a subdomain request
    };
    
    // Use the full path for subdomain-based proxying
    let proxy_path = original_uri.path();
    
    proxy_request(app_state, &connection_string, proxy_path, original_uri.clone(), method, headers, body).await
}

// R4.4 Core Proxy Logic
// Shared logic for both path-based and subdomain-based proxying
#[tracing::instrument(name = "proxy_request", skip(app_state, body, headers))]
async fn proxy_request(
    app_state: AppState,
    connection_string: &str,
    target_path: &str,
    original_uri: Uri,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, StatusCode> {
    // Look up the connection in the database
    let connection = sqlx::query_as::<_, Connection>(
        "SELECT id, connection_string, port, created_at FROM connections WHERE connection_string = ?",
    )
    .bind(connection_string)
    .fetch_optional(app_state.pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let connection = connection.ok_or(StatusCode::NOT_FOUND)?;

    // Establish or ensure holesail background connection is running
    ensure_background_connection(&connection.connection_string, connection.port as u16).await?;

    // Create the target URL with the correct path and query string
    let target_url = format!("http://localhost:{}{}", connection.port, target_path);
    
    // Preserve query string from original URI
    let final_url = if let Some(query) = original_uri.query() {
        format!("{}?{}", target_url, query)
    } else {
        target_url
    };

    println!("🔄 Proxying {} {} -> {}", method, connection_string, final_url);

    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert axum body to bytes
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Build the proxy request
    let mut request_builder = client.request(
        method.as_str().parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        &final_url,
    );

    // Forward relevant headers (excluding hop-by-hop headers)
    for (name, value) in headers.iter() {
        let name_str = name.as_str();
        if !is_hop_by_hop_header(name_str) && !name_str.starts_with("x-original-") {
            if let Ok(value_str) = value.to_str() {
                request_builder = request_builder.header(name_str, value_str);
            }
        }
    }

    // Add the body if present
    if !body_bytes.is_empty() {
        request_builder = request_builder.body(body_bytes);
    }

    // Send the request
    let response = request_builder
        .send()
        .await
        .map_err(|err| {
            println!("❌ Proxy request failed: {}", err);
            StatusCode::BAD_GATEWAY
        })?;

    // Build the response
    let status = StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    
    let mut response_builder = Response::builder().status(status);
    
    // Forward response headers (excluding hop-by-hop headers)
    for (name, value) in response.headers() {
        let name_str = name.as_str();
        if !is_hop_by_hop_header(name_str) {
            if let (Ok(header_name), Ok(header_value)) = (
                axum::http::HeaderName::from_bytes(name.as_str().as_bytes()),
                axum::http::HeaderValue::from_bytes(value.as_bytes())
            ) {
                response_builder = response_builder.header(header_name, header_value);
            }
        }
    }

    let body_bytes = response
        .bytes()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    response_builder
        .body(Body::from(body_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// R4.5 Background Connection Management
// Enhanced connection management using holesail's background features
async fn ensure_background_connection(connection_string: &str, port: u16) -> Result<(), StatusCode> {
    let connection_name = generate_connection_name(connection_string, port);
    let connection_key = format!("{}:{}", connection_string, port);

    // Update last used time
    {
        let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
        if let Some(conn) = connections.get_mut(&connection_key) {
            conn.last_used = std::time::SystemTime::now();
        }
    }

    // Check if connection exists and is running
    if is_background_connection_running(&connection_name).await? {
        tracing::debug!("Background connection {} already running", connection_name);
        
        // Update our tracking
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            if let Some(conn) = connections.get_mut(&connection_key) {
                conn.status = ConnectionStatus::Online;
            }
        }
        return Ok(());
    }

    // Check holesail availability
    let holesail_available = {
        let mut available = HOLESAIL_AVAILABLE.lock().unwrap();
        if available.is_none() {
            let is_available = check_holesail_available();
            *available = Some(is_available);
            is_available
        } else {
            available.unwrap()
        }
    };
    
    if !holesail_available {
        tracing::error!("❌ holesail command not available in PATH");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    // Start new background connection
    start_background_connection(connection_string, port, &connection_name, &connection_key).await
}

async fn start_background_connection(
    connection_string: &str,
    port: u16,
    connection_name: &str,
    connection_key: &str,
) -> Result<(), StatusCode> {
    tracing::info!("Starting background holesail connection: {}", connection_name);

    // Update tracking before starting
    {
        let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
        connections.insert(connection_key.to_string(), BackgroundConnection {
            connection_string: connection_string.to_string(),
            port,
            name: connection_name.to_string(),
            status: ConnectionStatus::Starting,
            last_used: std::time::SystemTime::now(),
        });
    }

    // Start holesail in background mode
    let output = TokioCommand::new("holesail")
        .arg(connection_string)
        .arg("--port")
        .arg(port.to_string())
        .arg("--background")
        .arg("--name")
        .arg(connection_name)
        .output()
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to start holesail background connection: {}", e);
            
            // Update status on failure
            {
                let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
                if let Some(conn) = connections.get_mut(connection_key) {
                    conn.status = ConnectionStatus::Error;
                }
            }
            
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("❌ Holesail background start failed: {}", stderr);
        
        // Update status on failure
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            if let Some(conn) = connections.get_mut(connection_key) {
                conn.status = ConnectionStatus::Error;
            }
        }
        
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Give the connection time to establish
    sleep(Duration::from_millis(2000)).await;

    // Verify connection is running
    if is_background_connection_running(connection_name).await? {
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            if let Some(conn) = connections.get_mut(connection_key) {
                conn.status = ConnectionStatus::Online;
            }
        }
        tracing::info!("✅ Background holesail connection established: {}", connection_name);
        Ok(())
    } else {
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            if let Some(conn) = connections.get_mut(connection_key) {
                conn.status = ConnectionStatus::Error;
            }
        }
        tracing::error!("❌ Background connection failed to start: {}", connection_name);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn is_background_connection_running(connection_name: &str) -> Result<bool, StatusCode> {
    let output = TokioCommand::new("holesail")
        .arg("--list")
        .output()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the specific format that holesail --list uses
    // Looking for lines that contain "Name:" followed by our connection name
    let mut current_connection_name = None;
    let mut current_status = None;
    
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("---") || line.starts_with("ID") {
            continue;
        }
        
        if line.starts_with("Name:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                current_connection_name = Some(parts[1].to_string());
            }
        } else if line.starts_with("Status:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                current_status = Some(parts[1].to_string());
            }
        }
        
        // If we have both name and status, check if it matches
        if let (Some(ref name), Some(ref status)) = (&current_connection_name, &current_status) {
            // Holesail adds "holesail-" prefix to our names
            // So "sando-d7925c15-5854" becomes "holesail-sando-d7925c15-5854"
            if name == &format!("holesail-{}", connection_name) && status == "online" {
                tracing::info!("✅ Found matching background connection: {} (status: {})", name, status);
                return Ok(true);
            }
            
            // Reset for next connection block
            current_connection_name = None;
            current_status = None;
        }
    }
    
    tracing::debug!("No matching online connection found for: {}", connection_name);
    Ok(false)
}

// R4.6 Holesail Availability Check
// Checks if holesail command is available in PATH
fn check_holesail_available() -> bool {
    match Command::new("holesail").arg("--help").output() {
        Ok(_) => {
            tracing::info!("✅ holesail command is available");
            true
        }
        Err(_) => {
            tracing::warn!("⚠️ holesail command not found in PATH");
            false
        }
    }
}

// R4.7 Connection Cleanup and Management
// Periodically clean up unused connections to prevent resource leaks
pub async fn cleanup_unused_connections() {
    let cleanup_threshold = Duration::from_secs(300); // 5 minutes
    
    loop {
        sleep(Duration::from_secs(60)).await; // Check every minute
        
        let mut connections_to_stop = Vec::new();
        
        {
            let connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            let now = std::time::SystemTime::now();
            
            for (key, conn) in connections.iter() {
                if conn.status == ConnectionStatus::Online {
                    if let Ok(elapsed) = now.duration_since(conn.last_used) {
                        if elapsed > cleanup_threshold {
                            connections_to_stop.push((key.clone(), conn.name.clone()));
                        }
                    }
                }
            }
        }
        
        for (key, name) in connections_to_stop {
            tracing::info!("🧹 Stopping unused background connection: {}", name);
            
            if let Err(e) = stop_background_connection(&name).await {
                tracing::error!("Failed to stop background connection {}: {:?}", name, e);
            } else {
                let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
                if let Some(conn) = connections.get_mut(&key) {
                    conn.status = ConnectionStatus::Stopped;
                }
            }
        }
    }
}

async fn stop_background_connection(connection_name: &str) -> Result<(), StatusCode> {
    let output = TokioCommand::new("holesail")
        .arg("--stop")
        .arg(connection_name)
        .output()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if output.status.success() {
        tracing::info!("✅ Stopped background connection: {}", connection_name);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("❌ Failed to stop background connection {}: {}", connection_name, stderr);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// R4.8 Helper Functions

// Generate a unique, safe name for background connections
fn generate_connection_name(connection_string: &str, port: u16) -> String {
    // Take first 8 characters of connection string for uniqueness
    let short_id = if connection_string.len() >= 8 {
        &connection_string[..8]
    } else {
        connection_string
    };
    
    format!("sando-{}-{}", short_id, port)
}

// Extract connection string from subdomain (e.g., "my-service.localhost:3000" -> "my-service")
fn extract_subdomain(host: &str, configured_host: &str) -> Result<String, StatusCode> {
    // Remove port if present
    let host_without_port = host.split(':').next().unwrap_or(host);
    
    // Check if it's a subdomain of the configured host
    let suffix = format!(".{}", configured_host);
    if let Some(subdomain) = host_without_port.strip_suffix(&suffix) {
        if !subdomain.is_empty() {
            Ok(subdomain.to_string())
        } else {
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

// Helper function to identify hop-by-hop headers that shouldn't be forwarded
fn is_hop_by_hop_header(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "connection" | "keep-alive" | "proxy-authenticate" | "proxy-authorization" 
        | "te" | "trailers" | "transfer-encoding" | "upgrade" | "host"
    )
}

// R4.9 Connection Status API
// Provides endpoints to check and manage background connections
pub async fn get_connection_status() -> Result<Response, StatusCode> {
    let connections = BACKGROUND_CONNECTIONS.lock().unwrap();
    let status_json = serde_json::to_string(&*connections)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(status_json))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// R4.10 Tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_generate_connection_name() {
        let name1 = generate_connection_name("abcdef123456789", 8080);
        assert_eq!(name1, "sando-abcdef12-8080");
        
        let name2 = generate_connection_name("short", 3000);
        assert_eq!(name2, "sando-short-3000");
    }

    #[test]
    async fn test_extract_subdomain() {
        // Test valid subdomains
        assert_eq!(extract_subdomain("api.localhost", "localhost"), Ok("api".to_string()));
        assert_eq!(extract_subdomain("my-service.localhost", "localhost"), Ok("my-service".to_string()));
        assert_eq!(extract_subdomain("test-api.localhost:3000", "localhost"), Ok("test-api".to_string()));
        
        // Test invalid cases
        assert!(extract_subdomain("localhost", "localhost").is_err());
        assert!(extract_subdomain(".localhost", "localhost").is_err());
        assert!(extract_subdomain("example.com", "localhost").is_err());
        assert!(extract_subdomain("", "localhost").is_err());
    }

    #[test]
    async fn test_is_hop_by_hop_header() {
        // Test hop-by-hop headers
        assert!(is_hop_by_hop_header("connection"));
        assert!(is_hop_by_hop_header("Connection"));
        assert!(is_hop_by_hop_header("HOST"));
        assert!(is_hop_by_hop_header("keep-alive"));
        assert!(is_hop_by_hop_header("transfer-encoding"));
        
        // Test non-hop-by-hop headers
        assert!(!is_hop_by_hop_header("content-type"));
        assert!(!is_hop_by_hop_header("authorization"));
        assert!(!is_hop_by_hop_header("user-agent"));
        assert!(!is_hop_by_hop_header("accept"));
    }

    #[test]
    async fn test_background_connection_tracking() {
        // Clear any existing connections for this test
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            connections.clear();
        }
        
        let connection_string = "test12345678";
        let port = 8080;
        let name = generate_connection_name(connection_string, port);
        let key = format!("{}:{}", connection_string, port);
        
        // Test that we can track a new connection
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            connections.insert(key.clone(), BackgroundConnection {
                connection_string: connection_string.to_string(),
                port,
                name: name.clone(),
                status: ConnectionStatus::Starting,
                last_used: std::time::SystemTime::now(),
            });
        }
        
        // Verify connection is tracked
        {
            let connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            assert!(connections.contains_key(&key));
            let conn = connections.get(&key).unwrap();
            assert_eq!(conn.connection_string, connection_string);
            assert_eq!(conn.port, port);
            assert_eq!(conn.status, ConnectionStatus::Starting);
        }
        
        // Test status update
        {
            let mut connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            if let Some(conn) = connections.get_mut(&key) {
                conn.status = ConnectionStatus::Online;
            }
        }
        
        // Verify status was updated
        {
            let connections = BACKGROUND_CONNECTIONS.lock().unwrap();
            let conn = connections.get(&key).unwrap();
            assert_eq!(conn.status, ConnectionStatus::Online);
        }
    }
}