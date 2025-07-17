/**
 * R1.0 Index Route
 * ================
 *
 * Handles requests to the root path (`/`).
 * This file is tagged for machine-readability.
 *
 * Tags: R1.1, R1.2
 */
// R1.1 Dependencies
use crate::components::home_page::home_page;
use axum::response::Html;

// R1.2 Index Handler
// Serves the main page by rendering the `home_page` component.
#[tracing::instrument(name = "index")]
pub async fn index() -> Html<String> {
    Html(home_page().into_string())
} 