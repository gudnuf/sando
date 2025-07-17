-- Sando Database Migration: 001
-- ===================================
--
-- Agent Instructions:
-- This file defines the database schema for the `connections` table.
-- The tag for this migration is `D1.1`.
--
-- D1.1: Create Connections Table

-- Create connections table
CREATE TABLE IF NOT EXISTS connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_string TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
); 