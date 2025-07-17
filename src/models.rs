/**
 * T1.0 Data Structures
 * ====================
 *
 * Defines the primary data structures used throughout the application.
 * This file is tagged for machine-readability.
 *
 * Tags: T1.1, T1.2, T1.3, T1.4, T1.5
 */
// T1.1 Dependencies
use serde::Deserialize;
use sqlx::FromRow;

// T1.2 ConnectionForm
// Represents the data submitted from the connection input form.
#[derive(Deserialize)]
pub struct ConnectionForm {
    pub connection: String,
    pub port: u16,
}

// T1.3 Connection
// Represents a single connection record retrieved from the database.
#[derive(FromRow)]
pub struct Connection {
    pub id: i64,
    pub connection_string: String,
    pub port: i32,
    pub created_at: String,
}