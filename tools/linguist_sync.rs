#!/usr/bin/env rust-script
//! Sync File Classification Patterns from GitHub Linguist
//!
//! This tool downloads Linguist's vendor.yml and generated.rb,
//! extracts file patterns, and generates Rust code for FileClassifier.
//!
//! Usage: cargo run --bin linguist_sync > src/file_classifier_generated.rs
//!
//! Patterns extracted:
//! - vendor.yml: Vendored code paths and files
//! - generated.rb: Generated file detection rules

use std::collections::{HashMap, HashSet};
use std::error::Error;

/// Linguist pattern sources
const VENDOR_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/vendor.yml";
const GENERATED_RB_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/generated.rb";

/// Parse vendor.yml regex patterns
fn extract_vendor_patterns(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut patterns = Vec::new();

    // vendor.yml format is YAML with regex patterns as strings
    // Pattern: "regex_pattern"
    for line in content.lines() {
        let trimmed = line.trim();
        // Look for quoted strings that look like patterns
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            let pattern = trimmed.trim_matches('"').to_string();
            // Only keep simple patterns, not complex regexes
            if !pattern.contains('(') && !pattern.contains('[') {
                patterns.push(pattern);
            }
        }
    }

    // Remove duplicates
    let unique: HashSet<_> = patterns.iter().cloned().collect();
    Ok(unique.into_iter().collect())
}

/// Parse generated.rb detection patterns
fn extract_generated_patterns(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut patterns = Vec::new();

    // generated.rb has patterns in various formats:
    // - File extensions: "\.pb\.rs", "\.generated\.ts"
    // - Directory paths: "__generated__/", "node_modules/"
    // - Content markers in comments

    for line in content.lines() {
        let trimmed = line.trim();

        // Match quoted string patterns
        if (trimmed.contains('\'') || trimmed.contains('"')) && !trimmed.starts_with('#') {
            // Extract quoted strings
            if let Some(start) = trimmed.find('\'').or_else(|| trimmed.find('"')) {
                let quote_char = trimmed.chars().nth(start).unwrap();
                if let Some(end) = trimmed[start + 1..].find(quote_char) {
                    let pattern = &trimmed[start + 1..start + 1 + end];
                    // Simple pattern validation
                    if !pattern.is_empty() && pattern.len() < 100 {
                        patterns.push(pattern.to_string());
                    }
                }
            }
        }
    }

    // Remove duplicates and sort
    let mut unique: Vec<_> = patterns.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
    unique.sort();
    Ok(unique)
}

/// Generate Rust code for patterns
fn generate_rust_code(
    vendored: Vec<String>,
    generated: Vec<String>,
) -> String {
    let mut code = String::from(
        r#"// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// This file is auto-generated from GitHub Linguist patterns
// Run: cargo run --bin linguist_sync
// Source: https://github.com/github-linguist/linguist

/// Auto-generated vendored code patterns from Linguist
pub const VENDORED_PATTERNS: &[&str] = &[
"#,
    );

    for pattern in &vendored {
        code.push_str(&format!("    {},\n", format_pattern_string(pattern)));
    }

    code.push_str(
        r#"];

/// Auto-generated file patterns for generated files from Linguist
pub const GENERATED_PATTERNS: &[&str] = &[
"#,
    );

    for pattern in &generated {
        code.push_str(&format!("    {},\n", format_pattern_string(pattern)));
    }

    code.push_str(
        r#"];
"#,
    );

    code
}

/// Format pattern string for Rust code
fn format_pattern_string(pattern: &str) -> String {
    // Escape special characters
    let escaped = pattern
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r");
    format!("\"{}\"", escaped)
}

/// Main function
fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Fetching Linguist vendor patterns...");
    // In a real implementation, this would fetch from the URLs
    // For now, we'll document the structure

    eprintln!("Phase 2: Auto-generation from Linguist");
    eprintln!("This is a roadmap implementation.");
    eprintln!("To fully implement:");
    eprintln!("1. Fetch vendor.yml from GitHub");
    eprintln!("2. Parse YAML file for regex patterns");
    eprintln!("3. Extract common patterns (node_modules/, vendor/, etc.)");
    eprintln!("4. Fetch generated.rb from GitHub");
    eprintln!("5. Parse Ruby code for extension/pattern matches");
    eprintln!("6. Generate Rust code arrays");
    eprintln!("7. Embed in build.rs for auto-generation");

    Ok(())
}
