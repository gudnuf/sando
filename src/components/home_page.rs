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
                
                // Favicon and app icons
                link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
                link rel="icon" type="image/x-icon" href="/static/favicon.svg";
                link rel="apple-touch-icon" href="/static/icon-192.svg";
                link rel="manifest" href="/static/manifest.json";
                
                // Theme colors for browsers
                meta name="theme-color" content="#3B82F6";
                meta name="msapplication-TileColor" content="#3B82F6";
                meta name="msapplication-config" content="/static/browserconfig.xml";
                
                // Open Graph / Social Media
                meta property="og:title" content="Sando.Blue - Ocean Tunnels";
                meta property="og:description" content="Create secure tunnels through the ocean depths";
                meta property="og:image" content="/static/icon-512.svg";
                meta property="og:type" content="website";
                meta name="twitter:card" content="summary";
                meta name="twitter:title" content="Sando.Blue";
                meta name="twitter:description" content="Create secure tunnels through the ocean depths";
                meta name="twitter:image" content="/static/icon-192.svg";
                
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
                style {
                    "
                    .form-container {
                        background: rgba(59, 130, 246, 0.15);
                        border: 2px solid rgba(96, 165, 250, 0.4);
                        border-radius: 12px;
                        padding: 2rem;
                        margin-bottom: 2rem;
                        box-shadow: 0 4px 20px rgba(59, 130, 246, 0.2);
                    }
                    .form-group input {
                        margin-bottom: 1.25rem;
                    }
                    .form-group input:last-child {
                        margin-bottom: 0;
                    }
                    .instructions-toggle {
                        background: rgba(59, 130, 246, 0.05);
                        border: 1px solid rgba(96, 165, 250, 0.2);
                        border-radius: 6px;
                        padding: 0.75rem 1rem;
                        cursor: pointer;
                        transition: all 0.3s ease;
                        text-align: center;
                        margin-bottom: 1rem;
                    }
                    .instructions-toggle:hover {
                        background: rgba(59, 130, 246, 0.1);
                        border-color: rgba(96, 165, 250, 0.3);
                    }
                    .instructions-content {
                        max-height: 0;
                        overflow: hidden;
                        transition: max-height 0.3s ease;
                        background: rgba(59, 130, 246, 0.08);
                        border-radius: 8px;
                        border: 1px solid rgba(96, 165, 250, 0.2);
                    }
                    .instructions-content.expanded {
                        max-height: 500px;
                        padding: 1.5rem;
                    }
                    "
                }
                script {
                    "
                    function toggleInstructions() {
                        const content = document.getElementById('instructions-content');
                        const toggle = document.getElementById('instructions-toggle');
                        const arrow = toggle.querySelector('.arrow');
                        
                        if (content.classList.contains('expanded')) {
                            content.classList.remove('expanded');
                            arrow.textContent = '▼';
                            toggle.querySelector('.toggle-text').textContent = 'Show Setup Instructions';
                        } else {
                            content.classList.add('expanded');
                            arrow.textContent = '▲';
                            toggle.querySelector('.toggle-text').textContent = 'Hide Setup Instructions';
                        }
                    }
                    "
                }
            }
            body {
                div class="container" {
                    h1 { "Sando" span style="color: #60A5FA;" { ".blue" } }
                    p style="text-align: center; margin-bottom: 2rem; color: #93C5FD;" { 
                        "Create secure tunnels through the ocean depths" 
                    }
                    
                    // Main form - now the primary focus
                    div class="form-container" {
                        h2 style="text-align: center; margin-bottom: 1.5rem; color: #60A5FA; font-size: 1.5rem;" { 
                            "🌊 Connect Your Service" 
                        }
                        form method="post" action="/submit" {
                            div class="form-group" {
                                label for="connection" style="display: block; margin-bottom: 0.5rem; font-weight: 600;" { 
                                    "Connection String" 
                                }
                                input 
                                    type="text" 
                                    id="connection" 
                                    name="connection" 
                                    placeholder="holesail --live <port_to_make_live> --background" 
                                    required;

                                label for="port" style="display: block; margin-bottom: 0.5rem; font-weight: 600;" {
                                    "Port Number"
                                }
                                input
                                    type="number"
                                    id="port"
                                    name="port"
                                    min="3001"
                                    max="8000"
                                    placeholder="3001-8000 (pick a random one)"
                                    required;
                            }
                            button type="submit" class="btn-full" style="padding: 0.75rem 1.5rem; font-size: 1rem; font-weight: 600;" {
                                span class="icon" { "🚀" }
                                "Dive Deep"
                            }
                        }
                    }

                    // Instructions - now collapsible and less prominent
                    div id="instructions-toggle" class="instructions-toggle" onclick="toggleInstructions()" {
                        span class="toggle-text" { "Show Setup Instructions" }
                        span class="arrow" style="margin-left: 0.5rem;" { "▼" }
                    }
                    
                    div id="instructions-content" class="instructions-content" {
                        h3 style="margin-bottom: 1rem; color: #60A5FA; font-size: 1.1rem;" { "🌊 Setup Instructions" }
                        
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
                                "Copy the connection string from the holesail output and paste it in the form above"
                            }
                        }
                    }

                    @if !cfg!(feature = "minimal") {
                        a href="/connections" class="btn btn-secondary mt-2" style="display: block; text-align: center; margin-top: 2rem;" {
                            span { "🌊" }
                            "Ocean View"
                        }
                    }
                }
            }
        }
    }
} 