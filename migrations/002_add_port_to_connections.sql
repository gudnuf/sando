-- Sando Database Migration: 002
-- ===================================
--
-- Agent Instructions:
-- This migration adds a port field to the connections table to support reverse proxy functionality.
-- The tag for this migration is D2.1.
--
-- D2.1: Add Port Column to Connections Table

-- Add port column to connections table
ALTER TABLE connections ADD COLUMN port INTEGER NOT NULL DEFAULT 8080; 