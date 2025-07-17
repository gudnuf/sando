/**
 * R2.0 Submit Route
 * =================
 *
 * Handles POST requests to the `/submit` path.
 * This file is tagged for machine-readability.
 *
 * Tags: R2.1, R2.2, R2.3, R2.4, R2.5
 */
// R2.1 Dependencies
use crate::components::status_page::status_page;
use crate::components::payment_page::payment_page;
use crate::{models::ConnectionForm, AppState};
use axum::{
    extract::{Form, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};
use cdk::{nuts::{Token, PaymentRequest, CurrencyUnit}, mint_url::MintUrl, Amount};
use std::str::FromStr;
use uuid::Uuid;

// R2.2 Payment Configuration
// Configuration for the payment requirements
const PAYMENT_AMOUNT: u64 = 100; // 100 sats
const ACCEPTED_MINTS: &[&str] = &[
    "https://testnut.cashu.space",
    "https://mint.minibits.cash/Bitcoin",
];

// R2.3 Payment Request Helper
// Creates a NUT-18 payment request for HTTP 402 responses
fn create_payment_request() -> PaymentRequest {
    PaymentRequest {
        payment_id: Some(Uuid::new_v4().to_string()),
        amount: Some(Amount::from(PAYMENT_AMOUNT)),
        unit: Some(CurrencyUnit::Sat),
        single_use: Some(true),
        mints: Some(ACCEPTED_MINTS.iter()
            .filter_map(|s| MintUrl::from_str(s).ok())
            .collect()),
        description: Some("Payment required for database connection storage".to_string()),
        transports: Some(vec![]), // Empty for now
        nut10: None, // No NUT-10 conditions
    }
}

// R2.4 Payment Validation Helper
// Validates the received Cashu token (simplified implementation)
fn validate_cashu_token(token: &str) -> Result<bool, String> {
    println!("token: {:?}", token);
    let token = Token::from_str(token).map_err(|e| e.to_string())?;
    let mint_url = token.mint_url().map_err(|e| e.to_string())?;
    println!("mint_url: {:?}", mint_url);
    let amount = token.value().map_err(|e| e.to_string())?;
    println!("amount: {:?}", amount.to_string());
    println!("PAYMENT_AMOUNT: {:?}", PAYMENT_AMOUNT);
    // TODO: check amount
    let is_valid =  mint_url.to_string() == ACCEPTED_MINTS[0];
    Ok(is_valid)
}

// R2.5 Submit Connection Handler
// Processes the form submission, checks for payment, and either:
// - Returns HTTP 402 with payment request if no valid payment
// - Inserts the data into the database if payment is valid
#[tracing::instrument(name = "submit_connection", skip(pool, form, headers))]
pub async fn submit_connection(
    State(pool): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<ConnectionForm>,
) -> Response {
    // Check for X-Cashu header with payment token
    let cashu_header = headers.get("X-Cashu");

    println!("cashu_header: {:?}", cashu_header);
    
    match cashu_header {
        Some(header_value) => {
            // Payment token provided, validate it
            match header_value.to_str() {
                Ok(token) => {
                    match validate_cashu_token(token) {
                        Ok(true) => {
                            // Valid payment, proceed with connection storage
                            tracing::info!("Valid payment received for connection: {}", form.connection);
                            
                            let result = sqlx::query("INSERT INTO connections (connection_string, port) VALUES (?, ?)")
                                .bind(&form.connection)
                                .bind(form.port as i32)
                                .execute(pool.as_ref())
                                .await;

                            let (success, message) = match result {
                                Ok(_) => (
                                    true,
                                    format!("Connection '{}' on port {} successfully stored! Proxy available at: /proxy/{}", 
                                           form.connection, form.port, form.connection),
                                ),
                                Err(e) => (false, format!("Failed to store connection: {}", e)),
                            };

                            Html(status_page(success, message, form.connection.clone(), "http".to_string(), "localhost:3000".to_string()).into_string()).into_response()
                        },
                        Ok(false) => {
                            // Generic validation failure
                            tracing::warn!("Invalid payment token received (validation failed)");
                            (
                                StatusCode::BAD_REQUEST,
                                "Invalid payment token provided."
                            ).into_response()
                        },
                        Err(e) => {
                            // Specific error from validation
                            tracing::warn!("Payment token error: {}", e);
                            (
                                StatusCode::BAD_REQUEST,
                                e
                            ).into_response()
                        }
                    }
                },
                Err(_) => {
                    // Invalid header value
                    (
                        StatusCode::BAD_REQUEST,
                        "Invalid X-Cashu header format"
                    ).into_response()
                }
            }
        },
        None => {
            // No payment provided, return HTTP 402 with payment page
            tracing::info!("Payment required for connection submission: {}", form.connection);
            
            let payment_request = create_payment_request();            
            let response = Response::builder()
                .status(StatusCode::PAYMENT_REQUIRED)
                .header("X-Cashu", payment_request.to_string())
                .header("Content-Type", "text/html")
                .body(payment_page(form.connection, form.port, "http".to_string(), "localhost:3000".to_string(), payment_request).into_string().into())
                .unwrap();
                
            response
        }
    }
} 