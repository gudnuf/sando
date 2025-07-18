/**
 * Sando Main Application
 * ======================
 *
 * Agent Instructions:
 * This file is documented for machine-readability. Use the table of contents
 * and search for the section number (e.g., "M1.2") to navigate.
 * Each section is tagged to link its purpose to the broader application structure.
 *
 * Table of Contents:
 * ------------------
 * M1. SETUP
 *     M1.1 Dependencies      (Imports)
 *     M1.2 Data Structures   (Structs & Types)
 *
 * M2. APPLICATION LOGIC
 *     M2.1 Routes            (Router Setup)
 *
 * M3. INITIALIZATION
 *     M3.1 Main Function     (Server Setup)
 */
// ==========================================================================
// M1. SETUP
// ==========================================================================

// M1.1 Dependencies
use axum::{
    body::Body,
    extract::{Host, OriginalUri, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tower::util::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod components;
mod models;
mod routes;

// M1.2 Data Structures
pub use models::{Connection};

#[derive(Clone)]
pub struct AppConfig {
    pub pool: Arc<SqlitePool>,
    pub host: String,
    pub port: u16,
}

pub type AppState = Arc<AppConfig>;

// ==========================================================================
// M2. APPLICATION LOGIC
// ==========================================================================

// M2.1 App Router
// This router handles the main application logic for non-proxy requests.
fn create_app_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/submit", post(routes::submit::submit_connection))
        .route("/connections", get(routes::connections::list_connections))
        .route("/connections/:id", delete(routes::connections::delete_connection))
        .route("/connections/batch-delete", post(routes::connections::batch_delete_connections))
        .route("/status/connections", get(routes::proxy::get_connection_status))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") })
}

// M2.2 Root Handler
// This is the main entry point for all incoming requests. It checks if the
// request is for a subdomain and either proxies it or forwards it to the main
// app router.
#[tracing::instrument(name = "root_handler", skip(app_state, request))]
async fn root_handler(
    State(app_state): State<AppState>,
    Host(host): Host,
    request: Request<Body>,
) -> Response {
    let host_without_port = host.split(':').next().unwrap_or(&host);

    // Check if the request is for a subdomain using the configured host.
    let base_host = &app_state.host;
    if host_without_port.ends_with(&format!(".{}", base_host)) && host_without_port != base_host {
        // It's a subdomain; let the proxy handler manage it.
        let (parts, body) = request.into_parts();
        
        match routes::proxy::proxy_handler_subdomain(
            State(app_state),
            Host(host),
            OriginalUri(parts.uri),
            parts.method,
            parts.headers,
            body,
        )
        .await
        {
            Ok(response) => response,
            Err(status_code) => Response::builder()
                .status(status_code)
                .body(Body::from(format!("Error: {}", status_code)))
                .unwrap(),
        }
    } else {
        // It's a standard request; forward it to the main app router.
        let app_router = create_app_router(app_state);
        match app_router.oneshot(request).await {
            Ok(response) => response.into_response(),
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap(),
        }
    }
}

// ==========================================================================
// M3. INITIALIZATION
// ==========================================================================

// M3.1 Main Function (Server Setup)
// Initializes the database, runs migrations, and starts the web server.
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("sando=debug,tower_http=debug,info")
        .init();

    // Create database pool
    let database_url = "sqlite:connections.db?mode=rwc";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let app_state = Arc::new(AppConfig {
        pool: Arc::new(pool),
        host: std::env::var("HOST").unwrap_or("localhost".to_string()),
        port,
    });

    // Start background cleanup task for holesail connections
    tokio::spawn(routes::proxy::cleanup_unused_connections());

    // The main router now uses a fallback to our root handler, which will
    // intelligently dispatch requests to either the proxy or the main app.
    let app = Router::new()
        .fallback(root_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state.clone());

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_state.port)).await.unwrap();
    
    println!("🚀 Server running on http://{}:{}", app_state.host, app_state.port);
    println!("📦 Database: connections.db");
    println!("🔄 Reverse proxy available at:");
    println!("   • Subdomain:  {{connection-string}}.{}:{}/{{path}}", app_state.host, app_state.port);
    
    // Run the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}