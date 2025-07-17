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
                title { "Sando.Blue - Tunnel Through the Ocean" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
            }
            body {
                div class="container" {
                    h1 { "Sando" span style="color: #60A5FA;" { ".blue" } }
                    p style="text-align: center; margin-bottom: 2rem; color: #93C5FD;" { 
                        "Create secure tunnels through the ocean depths" 
                    }
                    
                    div class="instructions-section" style="margin-bottom: 2rem; padding: 1.5rem; background: rgba(59, 130, 246, 0.1); border-radius: 8px; border: 1px solid rgba(96, 165, 250, 0.3);" {
                        h2 style="margin-bottom: 1rem; color: #60A5FA; font-size: 1.2rem;" { "🌊 Setup Instructions" }
                        
                        ol style="color: #E8F4FD; line-height: 1.6; padding-left: 1.5rem;" {
                            li style="margin-bottom: 0.75rem;" {
                                "Install Nix and Holesail following the instructions at "
                                a href="https://github.com/gudnuf/holesail-nix/tree/latest" 
                                  target="_blank" 
                                  rel="noopener noreferrer"
                                  style="color: #60A5FA; text-decoration: underline;" {
                                    "holesail-nix"
                                }
                            }
                            li style="margin-bottom: 0.75rem;" {
                                "Run this command to enter the development environment:"
                                div style="background: rgba(15, 23, 42, 0.8); padding: 0.75rem; border-radius: 4px; margin-top: 0.5rem; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace; font-size: 0.9rem; border: 1px solid rgba(71, 85, 105, 0.3);" {
                                    code style="color: #93C5FD; user-select: all;" {
                                        "nix --experimental-features 'nix-command flakes' develop github:gudnuf/holesail-nix/latest"
                                    }
                                }
                            }
                            li style="margin-bottom: 0.75rem;" {
                                "Start holesail with your local port (replace 3000 with your service port):"
                                div style="background: rgba(15, 23, 42, 0.8); padding: 0.75rem; border-radius: 4px; margin-top: 0.5rem; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace; font-size: 0.9rem; border: 1px solid rgba(71, 85, 105, 0.3);" {
                                    code style="color: #93C5FD; user-select: all;" {
                                        "holesail --live 3000 --background"
                                    }
                                }
                            }
                            li {
                                "Copy the connection string from the holesail output and paste it in the form below"
                            }
                        }
                    }

                    form method="post" action="/submit" {
                        div class="form-group" {
                            label for="connection" { 
                                "🌊 Connection String" 
                            }
                            input 
                                type="text" 
                                id="connection" 
                                name="connection" 
                                placeholder="Paste your holesail connection string here" 
                                required;

                            input
                                type="number"
                                id="port"
                                name="port"
                                min="3001"
                                max="8000"
                                placeholder="3001-8000 (pick a random one)"
                                required;

                        }
                        button type="submit" {
                            span class="icon" { "🚀" }
                            "Dive Deep"
                        }
                    }
                    @if !cfg!(feature = "minimal") {
                        a href="/connections" class="btn btn-secondary mt-2" {
                            span { "🌊" }
                            "Ocean View"
                        }
                    }
                }
            }
        }
    }
} 