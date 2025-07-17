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
pub type AppState = Arc<SqlitePool>;

// ==========================================================================
// M2. APPLICATION LOGIC
// ==========================================================================

// M2.1 App Router
// This router handles the main application logic for non-proxy requests.
fn create_app_router(pool: AppState) -> Router {
    Router::new()
        .route("/", get(routes::index::index))
        .route("/submit", post(routes::submit::submit_connection))
        .route("/connections", get(routes::connections::list_connections))
        .route("/connections/:id", delete(routes::connections::delete_connection))
        .route("/connections/batch-delete", post(routes::connections::batch_delete_connections))
        .route("/status/connections", get(routes::proxy::get_connection_status))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(pool)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") })
}

// M2.2 Root Handler
// This is the main entry point for all incoming requests. It checks if the
// request is for a subdomain and either proxies it or forwards it to the main
// app router.
#[tracing::instrument(name = "root_handler", skip(pool, request))]
async fn root_handler(
    State(pool): State<AppState>,
    Host(host): Host,
    request: Request<Body>,
) -> Response {
    let host_without_port = host.split(':').next().unwrap_or(&host);

    // Check if the request is for a subdomain.
    if host_without_port.ends_with(".localhost") && host_without_port != "localhost" {
        // It's a subdomain; let the proxy handler manage it.
        let (parts, body) = request.into_parts();
        
        match routes::proxy::proxy_handler_subdomain(
            State(pool),
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
        let app_router = create_app_router(pool);
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

    let app_state = Arc::new(pool);

    // Start background cleanup task for holesail connections
    tokio::spawn(routes::proxy::cleanup_unused_connections());

    // The main router now uses a fallback to our root handler, which will
    // intelligently dispatch requests to either the proxy or the main app.
    let app = Router::new()
        .fallback(root_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let port: u16 = 3000;
    let host: String = "localhost".to_string();

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    
    println!("🚀 Server running on http://{}:{}", host, port);
    println!("📦 Database: connections.db");
    println!("🔄 Reverse proxy available at:");
    println!("   • Subdomain:  {{connection-string}}.{}:{}/{{path}}", host, port);
    
    // Run the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}