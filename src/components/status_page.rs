/**
 * C2.0 Status Page Component
 * ==========================
 *
 * Renders a status page indicating the success or failure of a
 * connection string submission.
 * This file is tagged for machine-readability.
 *
 * Tags: C2.1, C2.2
 */
// C2.1 Dependencies
use maud::{html, Markup, DOCTYPE};

// C2.2 Status Page Function
// Generates the Maud Markup for the status page, dynamically changing
// content based on the success flag and message.
pub fn status_page(success: bool, message: String, subdomain: String, protocol: String, host: String) -> Markup {
    let service_url = format!("{}://{}.{}", protocol, subdomain, host);
    
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Sando - Status" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
            }
            body {
                div class="container status-container" {
                    @if success {
                        div class="status-icon status-icon-success" { "✓" }
                        h1 { "Tunnel Created" }
                        
                        div class="service-link-container" {
                            a href=(service_url) 
                              target="_blank" 
                              rel="noopener noreferrer"
                              class="service-link" {
                                (service_url)
                            }
                            p class="service-description" { 
                                "Click the link above to access your proxied service."
                            }
                        }
                        
                        div class="actions" {
                            a href="/" class="btn btn-secondary" { "＋ New Tunnel" }
                            a href="/connections" class="btn btn-secondary" { "☰ View All" }
                        }
                    } @else {
                        div class="status-icon status-icon-error" { "✗" }
                        h1 { "Tunnel Failed" }
                        @if !message.is_empty() {
                            p class="error-message" { (message) }
                        }
                        div class="actions" {
                            a href="/" class="btn" { "❮ Try Again" }
                        }
                    }
                }
            }
        }
    }
} 