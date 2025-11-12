//! Synchronize File Classification and Language Detection Patterns from GitHub Linguist
//!
//! This tool downloads Linguist's pattern files and generates Rust code for:
//! - Phase 2: File classification (vendor, generated, binary patterns)
//! - Phase 3: Language detection heuristics (ambiguous extension resolution)
//!
//! Run with: cargo run --bin sync-linguist --features sync-tool
//!
//! Output: Generates two files:
//! - src/file_classifier_generated.rs (Phase 2)
//! - src/heuristics_generated.rs (Phase 3)

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};

const VENDOR_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/vendor.yml";
const GENERATED_RB_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/generated.rb";
const HEURISTICS_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/heuristics.yml";

/// Fetch content from a URL
async fn fetch_url(url: &str) -> Result<String> {
    eprintln!("📥 Fetching {}", url);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .context(format!("Failed to fetch {}", url))?;

    let content = response
        .text()
        .await
        .context("Failed to read response body")?;

    eprintln!("✅ Fetched {} bytes", content.len());
    Ok(content)
}

/// Parse vendor.yml and extract vendored path patterns
fn parse_vendor_yml(content: &str) -> HashSet<String> {
    let mut patterns = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Extract patterns (YAML list items start with -)
        if let Some(pattern) = trimmed.strip_prefix("- ") {
            let cleaned = pattern
                .trim_matches('"')
                .trim_matches('\'')
                .trim()
                .to_string();

            if !cleaned.is_empty() && !cleaned.contains('(') && cleaned.len() < 100 {
                let _ = patterns.insert(cleaned);
            }
        }
    }

    patterns
}

/// Parse generated.rb and extract generated file patterns
fn parse_generated_rb(content: &str) -> HashSet<String> {
    let mut patterns = HashSet::new();
    let quote_pattern = Regex::new(r#"['"](.*?)['"]"#).unwrap();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Extract quoted strings
        for cap in quote_pattern.captures_iter(trimmed) {
            if let Some(matched) = cap.get(1) {
                let pattern = matched.as_str().to_string();
                if pattern.len() < 100 && !pattern.contains("//") {
                    let _ = patterns.insert(pattern);
                }
            }
        }
    }

    patterns
}

/// Parse heuristics.yml and extract language detection rules (Phase 3)
fn parse_heuristics_yml(content: &str) -> HashMap<String, Vec<String>> {
    let mut heuristics: HashMap<String, Vec<String>> = HashMap::new();

    // Simple parser for disambiguation rules
    let mut current_extension: Option<String> = None;
    let mut patterns: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        // Detect disambiguation blocks (extensions like ".pl:", ".m:", etc.)
        if trimmed.ends_with(':') && !trimmed.starts_with('-') {
            // Save previous extension's patterns
            if let Some(ext) = current_extension.take() {
                if !patterns.is_empty() {
                    let _ = heuristics.insert(ext, patterns.clone());
                    patterns.clear();
                }
            }

            current_extension = Some(trimmed.trim_end_matches(':').trim_matches('"').to_string());
        }

        // Extract pattern content (quoted strings)
        if trimmed.contains('"') {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let pattern = &trimmed[start + 1..start + 1 + end];
                    if !pattern.is_empty() && pattern.len() < 200 {
                        patterns.push(pattern.to_string());
                    }
                }
            }
        }
    }

    // Save last extension
    if let Some(ext) = current_extension {
        if !patterns.is_empty() {
            let _ = heuristics.insert(ext, patterns);
        }
    }

    heuristics
}

/// Categorize patterns into vendored, generated, and binary
fn categorize_patterns(
    patterns: &HashSet<String>,
) -> (HashSet<String>, HashSet<String>, HashSet<String>) {
    let mut vendored = HashSet::new();
    let mut generated = HashSet::new();
    let mut binary = HashSet::new();

    let binary_exts = [
        ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".svg", ".zip", ".tar", ".gz", ".rar", ".exe",
        ".dll", ".pdf", ".mp3", ".mp4",
    ];

    for pattern in patterns {
        if pattern.starts_with('.') && pattern.len() < 10 {
            // It's an extension
            if binary_exts.contains(&pattern.as_str()) {
                let _ = binary.insert(pattern.clone());
            } else if pattern.contains("pb")
                || pattern.contains("generated")
                || pattern.contains("proto")
            {
                let _ = generated.insert(pattern.clone());
            } else {
                // Explicitly ignore other extension-like entries for clarity.
            }
        } else {
            // It's a path
            let _ = vendored.insert(pattern.clone());
        }
    }

    (vendored, generated, binary)
}

/// Generate Rust code for file classifier patterns (Phase 2)
fn generate_file_classifier_code(
    vendored: &HashSet<String>,
    generated: &HashSet<String>,
    binary: &HashSet<String>,
) -> String {
    let mut code = String::from(
        r"// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// Generated from GitHub Linguist patterns (vendor.yml, generated.rb)
// Run: cargo run --bin sync-linguist --features sync-tool

//! Auto-generated file classification patterns from GitHub Linguist
//!
//! These patterns are kept in sync with GitHub Linguist via:
//! cargo run --bin sync-linguist --features sync-tool
//!
//! Sources:
//! - vendor.yml: Vendored code detection
//! - generated.rb: Auto-generated file detection

/// Vendored code path patterns (from Linguist vendor.yml)
pub const VENDORED_PATTERNS_FROM_LINGUIST: &[&str] = &[
",
    );

    let mut sorted_vendored: Vec<_> = vendored.iter().collect();
    sorted_vendored.sort();
    for pattern in sorted_vendored {
        let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!("    \"{}\",\n", escaped));
    }

    code.push_str("];\n\n");

/// Generated file patterns (from Linguist generated.rb)
    pub const GENERATED_PATTERNS_FROM_LINGUIST: &[&str] = &[
",
    );

    let mut sorted_generated: Vec<_> = generated.iter().collect();
    sorted_generated.sort();
    for pattern in sorted_generated {
        let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
        code.push_str(&format!("    \"{}\",\n", escaped));
    }

    code.push_str("];\n\n");

/// Binary file extensions (images, archives, executables, documents, media)
    pub const BINARY_PATTERNS_FROM_LINGUIST: &[&str] = &[
",
    );

    let mut sorted_binary: Vec<_> = binary.iter().collect();
    sorted_binary.sort();
    for pattern in sorted_binary {
        code.push_str(&format!("    \"{}\",\n", pattern));
    }

    code.push_str("];\n");
    code
}

/// Generate Rust code for language detection heuristics (Phase 3)
fn generate_heuristics_code(heuristics: &HashMap<String, Vec<String>>) -> String {
    let mut code = String::from(
        r"// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// Generated from GitHub Linguist heuristics.yml (Phase 3)
// Run: cargo run --bin sync-linguist --features sync-tool

//! Auto-generated language detection heuristics from GitHub Linguist
//!
//! These patterns help detect languages for ambiguous file extensions:
//! - .pl: Perl vs Prolog
//! - .m: Objective-C vs Matlab
//! - .rs: Rust (resolved via extension only, but kept for future use)
//!
//! Implementation roadmap:
//! 1. Extract patterns from heuristics.yml
//! 2. Build decision tree for ambiguous extensions
//! 3. Integrate into language detection pipeline

/// Language detection heuristics (Phase 3)
/// Maps file extensions to detection patterns
pub const LANGUAGE_DETECTION_HEURISTICS: &[(&str, &[&str])] = &[
",
    );

    let mut sorted_exts: Vec<_> = heuristics.keys().collect();
    sorted_exts.sort();

    for ext in sorted_exts {
        if let Some(patterns) = heuristics.get(ext) {
            code.push_str(&format!("    (\"{}\", &[\n", ext));
            for pattern in patterns.iter().take(5) {
                // Limit to 5 patterns per extension
                let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
                code.push_str(&format!("        \"{}\",\n", escaped));
            }
            code.push_str("    ]),\n");
        }
    }

    code.push_str("];\n");
    code
}

/// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("🚀 Syncing patterns from GitHub Linguist...\n");

    // Fetch files
    let vendor_content = fetch_url(VENDOR_YML_URL).await?;
    let generated_content = fetch_url(GENERATED_RB_URL).await?;
    let heuristics_content = fetch_url(HEURISTICS_YML_URL).await?;

    eprintln!("\n⚙️  Parsing patterns...\n");

    // Parse Phase 2 patterns
    let vendor_patterns = parse_vendor_yml(&vendor_content);
    let generated_patterns = parse_generated_rb(&generated_content);

    eprintln!("Found {} vendor patterns", vendor_patterns.len());
    eprintln!("Found {} generated patterns", generated_patterns.len());

    let all_patterns = vendor_patterns
        .union(&generated_patterns)
        .cloned()
        .collect();
    let (vendored, generated, binary) = categorize_patterns(&all_patterns);

    eprintln!(
        "Categorized: {} vendored, {} generated, {} binary\n",
        vendored.len(),
        generated.len(),
        binary.len()
    );

    // Parse Phase 3 heuristics
    let heuristics = parse_heuristics_yml(&heuristics_content);
    eprintln!(
        "Found {} language detection heuristics (Phase 3)\n",
        heuristics.len()
    );

    // Generate code
    eprintln!("💾 Generating Rust code...\n");

    let file_classifier_code = generate_file_classifier_code(&vendored, &generated, &binary);
    let heuristics_code = generate_heuristics_code(&heuristics);

    // Output Phase 2
    println!("{}", file_classifier_code);
    eprintln!("✅ Generated file classifier patterns (Phase 2)");

    // Output Phase 3 to stderr (user will redirect)
    eprintln!("\n{}", heuristics_code);
    eprintln!("✅ Generated language detection heuristics (Phase 3)");

    eprintln!("\n📍 Complete! Patterns synced from GitHub Linguist");
    eprintln!("   Phase 2: File classification");
    eprintln!("   Phase 3: Language detection heuristics");

    Ok(())
}
