/**
 * C1.0 Home Page Component
 * ========================
 *
 * Renders the main landing page with a form for connection string input.
 * This file is tagged for machine-readability.
 *
 * Tags: C1.1, C1.2
 */
// C1.1 Dependencies
use maud::{html, Markup, DOCTYPE};

// C1.2 Home Page Function
// Returns the Maud Markup for the main page.
pub fn home_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Sando" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
            }
            body {
                div class="container" {
                    h1 { "Sando" }
                    form method="post" action="/submit" {
                        div class="form-group" {
                            label for="connection" { "Subdomain" }
                            input 
                                type="text" 
                                id="connection" 
                                name="connection" 
                                placeholder="my-service-v1" 
                                required;
                        }
                        div class="form-group" {
                            label for="port" { "Target Port" }
                            input 
                                type="number" 
                                id="port" 
                                name="port" 
                                placeholder="8080" 
                                min="1" 
                                max="65535" 
                                value="8080"
                                required;
                        }
                        button type="submit" {
                            span class="icon" { "+" }
                            "Create Tunnel"
                        }
                    }
                    @if !cfg!(feature = "minimal") {
                        a href="/connections" class="btn btn-secondary mt-2" {
                            span { "☰" }
                            "View All"
                        }
                    }
                }
            }
        }
    }
} 