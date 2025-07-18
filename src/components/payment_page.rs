/**
 * C4.0 Payment Page Component
 * ===========================
 *
 * Displays a payment form when HTTP 402 is returned.
 * Allows users to input their Cashu token and resubmit.
 * This file is tagged for machine-readability.
 *
 * Tags: C4.1, C4.2, C4.3
 */

// C4.1 Dependencies
use maud::{html, Markup, PreEscaped, DOCTYPE};
use cdk::nuts::PaymentRequest;

// C4.2 Payment Page Component
// Renders a page with a form for users to input their Cashu token
pub fn payment_page(connection_string: String, port: u16, protocol: String, host: String, payment_request: PaymentRequest) -> Markup {
    let service_url = format!("{}://{}.{}", protocol, connection_string, host);
    
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Ocean Toll - Sando.Blue" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

            }
            body {
                div class="payment-container" {
                    div class="error-header" {
                        div class="error-icon" { "🏴‍☠️" }
                        h1 { "Ocean Toll Required" }
                    }
                    
                    div class="payment-details" {
                        div class="amount" { (payment_request.amount.unwrap().to_string()) " sats" }
                        p class="service-info" { 
                            "⚓ Destination: " code { (service_url) }
                        }
                    }
                    
                    form id="payment-form" method="POST" action="/submit" class="payment-form" {
                        input type="hidden" name="connection" value=(connection_string);
                        input type="hidden" name="port" value=(port);
                        
                        div class="form-group" {
                            label for="cashu-token" { 
                                "🪙 Cashu Token" 
                            }
                            textarea 
                                id="cashu-token" 
                                name="cashu_token" 
                                class="token-input"
                                placeholder="cashuAeyJ0b2tlbiI6W3sicHJvb2ZzI..."
                                required {}
                        }
                        
                        div id="status-message" class="status-message" style="display: none;" {}
                        
                        div class="actions mt-4" {
                            button type="submit" class="btn btn-full" { "🌊 Pay Passage" }
                            a href="/" class="btn btn-cancel mt-2" { "🏃‍♂️ Abandon Ship" }
                        }
                    }
                }
                
                script { (PreEscaped(payment_script())) }
            }
        }
    }
}

// C4.3 Payment Form JavaScript
// Handles form submission for payment processing
fn payment_script() -> String {
    r#"
        const tokenInput = document.getElementById('cashu-token');
        const form = document.getElementById('payment-form');
        const statusMessage = document.getElementById('status-message');
        const submitBtn = form.querySelector('button[type="submit"]');
        
        function showStatus(message, isError = false) {
            statusMessage.textContent = message;
            statusMessage.className = 'status-message ' + (isError ? 'status-error' : 'status-success');
            statusMessage.style.display = 'block';
        }
        
        function hideStatus() {
            statusMessage.style.display = 'none';
        }
        
        form.addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const cashuToken = tokenInput.value.trim();
            
            if (!cashuToken) {
                showStatus('🪙 Please paste your Cashu token to pay the ocean toll.', true);
                return;
            }
            
            if (!cashuToken.startsWith('cashu')) {
                showStatus('⚠️ Invalid token format. It must start with "cashuA".', true);
                return;
            }
            
            // Show processing state
            form.classList.add('processing');
            submitBtn.disabled = true;
            showStatus('🌊 Navigating through the payment waters...');
            
            try {
                const formData = new FormData(form);
                
                const response = await fetch('/submit', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/x-www-form-urlencoded',
                        'X-Cashu': cashuToken
                    },
                    body: new URLSearchParams({
                        connection: formData.get('connection'),
                        port: formData.get('port')
                    })
                });
                
                if (response.ok) {
                    showStatus('⚓ Passage paid! Setting sail to your destination...', false);
                    const responseText = await response.text();
                    // Disable further interaction while redirecting
                    form.querySelectorAll('input, textarea, button').forEach(el => el.disabled = true);
                    
                    setTimeout(() => {
                        document.open();
                        document.write(responseText);
                        document.close();
                    }, 1500);
                } else {
                    const errorText = await response.text();
                    showStatus(`🌪️ Payment failed: ${errorText || 'Please check your token and try again.'}`, true);
                }
            } catch (error) {
                console.error('Payment error:', error);
                showStatus('🌊 The ocean depths swallowed your request. Please try again.', true);
            } finally {
                form.classList.remove('processing');
                submitBtn.disabled = false;
            }
        });
        
        // Clear status when user starts typing
        tokenInput.addEventListener('input', function() {
            hideStatus();
        });
    "#.to_string()
} 