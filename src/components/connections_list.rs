/**
 * C3.0 Connections List Component
 * ===============================
 *
 * Renders a list of all connections stored in the database.
 * This file is tagged for machine-readability.
 *
 * Tags: C3.1, C3.2
 */
// C3.1 Dependencies
use crate::Connection;
use maud::{html, Markup, DOCTYPE};

// C3.2 Connections List Function
// Generates the Maud Markup for the connections list page.
pub fn connections_list(connections: &[Connection]) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Sando - Connections" }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet";
                link rel="stylesheet" href="/static/styles.css";
                style {
                    "
                    input[type='checkbox'] {
                        width: 16px !important;
                        height: 16px !important;
                        min-width: 16px !important;
                        min-height: 16px !important;
                        flex-shrink: 0 !important;
                        cursor: pointer;
                    }
                    
                    .checkbox-label {
                        display: flex !important;
                        align-items: center !important;
                        gap: 0.5rem !important;
                    }
                    
                    .connection-header {
                        display: flex !important;
                        align-items: center !important;
                        gap: 1rem !important;
                    }
                    
                    .connection-header input[type='checkbox'] {
                        margin: 0 !important;
                    }
                    "
                }
            }
            body {
                div class="container container-wide" {
                    h1 { "Active Tunnels" }
                    @if connections.is_empty() {
                        div class="empty-state" {
                            p { "No active tunnels. Create one to get started." }
                            a href="/" class="btn" { 
                                span { "+" }
                                "Create Tunnel" 
                            }
                        }
                    } @else {
                        div class="connections-controls" {
                            div class="selection-controls" {
                                label class="checkbox-label" {
                                    input type="checkbox" id="select-all" onchange="toggleSelectAll()";
                                    span { "Select All" }
                                }
                                button type="button" class="btn btn-danger btn-small" id="batch-delete-btn" onclick="deleteBatch()" disabled { 
                                    span { "🗑️" }
                                    "Delete" 
                                }
                            }
                            div class="connection-count" {
                                span id="selected-count" { "0" }
                                " of "
                                (connections.len())
                                " selected"
                            }
                        }
                        
                        form id="batch-delete-form" method="post" action="/connections/batch-delete" style="display: none;" {}
                        
                        div class="connections-list" {
                            @for connection in connections {
                                div class="connection-item" {
                                    div class="connection-header" {
                                        input type="checkbox" class="connection-select" value=(connection.id) onchange="updateSelection()";
                                        div class="connection-info" {
                                            @let truncated_connection = if connection.connection_string.len() > 20 {
                                                format!("{}...", &connection.connection_string[..20])
                                            } else {
                                                connection.connection_string.clone()
                                            };
                                            @let full_url = format!("http://{}.localhost:3000", connection.connection_string);
                                            a href=(full_url) target="_blank" title=(format!("{}.localhost:3000", connection.connection_string)) {
                                                (truncated_connection) ".localhost:3000"
                                            }
                                        }
                                        div class="connection-actions" {
                                            button type="button" class="btn btn-small btn-danger" onclick={ "deleteConnection(" (connection.id) ")" } { "🗑️" }
                                        }
                                    }
                                    div class="connection-date" {
                                        "Created: " (connection.created_at)
                                    }
                                }
                            }
                        }
                        div class="actions mt-4" {
                            a href="/" class="btn btn-secondary" { 
                                span { "❮" }
                                "Back" 
                            }
                        }
                    }
                }
                
                script {
                    (maud::PreEscaped(r#"
                        function toggleSelectAll() {
                            const selectAll = document.getElementById('select-all');
                            const checkboxes = document.querySelectorAll('.connection-select');
                            checkboxes.forEach(cb => cb.checked = selectAll.checked);
                            updateSelection();
                        }
                        
                        function updateSelection() {
                            const checkboxes = document.querySelectorAll('.connection-select');
                            const selectedCheckboxes = document.querySelectorAll('.connection-select:checked');
                            const selectAll = document.getElementById('select-all');
                            const selectedCount = document.getElementById('selected-count');
                            const batchDeleteBtn = document.getElementById('batch-delete-btn');
                            
                            if (!selectAll || !selectedCount || !batchDeleteBtn) return;

                            // Update count
                            selectedCount.textContent = selectedCheckboxes.length;
                            
                            // Update select all checkbox state
                            if (selectedCheckboxes.length === 0) {
                                selectAll.indeterminate = false;
                                selectAll.checked = false;
                            } else if (selectedCheckboxes.length === checkboxes.length) {
                                selectAll.indeterminate = false;
                                selectAll.checked = true;
                            } else {
                                selectAll.indeterminate = true;
                                selectAll.checked = false;
                            }
                            
                            // Enable/disable batch delete button
                            batchDeleteBtn.disabled = selectedCheckboxes.length === 0;
                        }
                        
                        function deleteBatch() {
                            const selectedCheckboxes = document.querySelectorAll('.connection-select:checked');
                            if (selectedCheckboxes.length === 0) return;
                            
                            const count = selectedCheckboxes.length;
                            if (!confirm(`Delete ${count} tunnel${count > 1 ? 's' : ''}?`)) return;
                            
                            const form = document.getElementById('batch-delete-form');
                            form.innerHTML = '';
                            
                            // Create a comma-separated string of IDs
                            const ids = Array.from(selectedCheckboxes).map(cb => cb.value).join(',');
                            const input = document.createElement('input');
                            input.type = 'hidden';
                            input.name = 'connection_ids';
                            input.value = ids;
                            form.appendChild(input);
                            
                            form.submit();
                        }
                        
                        function deleteConnection(id) {
                            if (!confirm('Delete this tunnel?')) return;
                            
                            fetch(`/connections/${id}`, { 
                                method: 'DELETE',
                                headers: {
                                    'Content-Type': 'application/json',
                                }
                            })
                            .then(response => {
                                if (response.ok) {
                                    window.location.reload();
                                } else {
                                    alert('Failed to delete tunnel');
                                }
                            })
                            .catch(err => {
                                console.error('Delete failed:', err);
                                alert('Failed to delete tunnel');
                            });
                        }

                        // Initial state update
                        document.addEventListener('DOMContentLoaded', updateSelection);
                    "#))
                }
            }
        }
    }
} 