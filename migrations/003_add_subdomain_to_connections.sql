-- Sando Database Migration: 003
-- ===================================
--
-- Agent Instructions:
-- This migration adds a subdomain field to the connections table to support custom subdomain routing.
-- The tag for this migration is D3.1.
--
-- D3.1: Add Subdomain Column to Connections Table

-- Add subdomain column to connections table (nullable initially for existing records)
ALTER TABLE connections ADD COLUMN subdomain TEXT; 