/**
 * R3.0 Connections Route
 * ======================
 *
 * Handles GET requests to the `/connections` path.
 * This file is tagged for machine-readability.
 *
 * Tags: R3.1, R3.2, R3.3, R3.4
 */
// R3.1 Dependencies
use crate::components::connections_list::connections_list;
use crate::{AppState, Connection};
use axum::{extract::{Path, State}, http::StatusCode, response::{Html, Redirect}, Form};
use serde::Deserialize;

// R3.2 List Connections Handler
// Fetches all connections from the database and renders the
// `connections_list` component.
#[tracing::instrument(name = "list_connections", skip(app_state))]
pub async fn list_connections(State(app_state): State<AppState>) -> Html<String> {
    let connections = sqlx::query_as::<_, Connection>(
        "SELECT id, connection_string, port, created_at FROM connections ORDER BY created_at DESC",
    )
    .fetch_all(app_state.pool.as_ref())
    .await
    .unwrap_or_else(|_| vec![]);

    Html(connections_list(&connections, &app_state.host, app_state.port).into_string())
}

// R3.3 Delete Single Connection Handler
// Deletes a single connection by ID and redirects back to the connections list
#[tracing::instrument(name = "delete_connection", skip(app_state))]
pub async fn delete_connection(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, StatusCode> {
    sqlx::query("DELETE FROM connections WHERE id = ?")
        .bind(id)
        .execute(app_state.pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/connections"))
}

// R3.4 Batch Delete Form Structure
#[derive(Deserialize, Debug)]
pub struct BatchDeleteForm {
    pub connection_ids: String,  // Comma-separated string of IDs
}

// R3.5 Batch Delete Handler
// Deletes multiple connections and redirects back to the connections list
#[tracing::instrument(name = "batch_delete_connections", skip(app_state))]
pub async fn batch_delete_connections(
    State(app_state): State<AppState>,
    Form(form): Form<BatchDeleteForm>,
) -> Result<Redirect, StatusCode> {
    // Parse comma-separated string of IDs
    let connection_ids: Result<Vec<i64>, _> = form.connection_ids
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|id_str| id_str.trim().parse::<i64>())
        .collect();

    let connection_ids = connection_ids.map_err(|_| StatusCode::BAD_REQUEST)?;

    if connection_ids.is_empty() {
        return Ok(Redirect::to("/connections"));
    }

    // Create placeholders for the IN clause
    let placeholders = vec!["?"; connection_ids.len()].join(",");
    let query_str = format!("DELETE FROM connections WHERE id IN ({})", placeholders);
    
    let mut query = sqlx::query(&query_str);
    for id in connection_ids {
        query = query.bind(id);
    }
    
    query
        .execute(app_state.pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/connections"))
} 