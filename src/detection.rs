//! Language Detection Utilities
//!
//! Advanced language detection beyond simple extension matching.

use crate::registry::{LanguageInfo, LANGUAGE_REGISTRY};
use std::path::Path;

/// Detect language from file content (shebang, magic bytes, etc.)
///
/// Returns None if the content doesn't match any supported language.
/// No fallback - explicit detection only.
#[must_use]
pub fn detect_from_content(content: &str) -> Option<&'static LanguageInfo> {
    // Check shebang first
    if let Some(lang) = detect_from_shebang(content) {
        return Some(lang);
    }

    // Check for specific content patterns
    if let Some(lang) = detect_from_patterns(content) {
        return Some(lang);
    }

    // No fallback - if not detected, return None
    None
}

/// Detect language from shebang line
pub fn detect_from_shebang(content: &str) -> Option<&'static LanguageInfo> {
    let first_line = content.lines().next()?;
    if !first_line.starts_with("#!") {
        return None;
    }

    let shebang = first_line.to_lowercase();

    // Map common interpreters to languages
    if shebang.contains("python") {
        LANGUAGE_REGISTRY.get_language("python")
    } else if shebang.contains("node") || shebang.contains("js") {
        LANGUAGE_REGISTRY.get_language("javascript")
    } else if shebang.contains("ruby") || shebang.contains("rb") {
        LANGUAGE_REGISTRY.get_language("ruby")
    } else if shebang.contains("bash") || shebang.contains("/sh") {
        LANGUAGE_REGISTRY.get_language("bash")
    } else if shebang.contains("elixir") {
        LANGUAGE_REGISTRY.get_language("elixir")
    } else if shebang.contains("erlang") || shebang.contains("escript") {
        LANGUAGE_REGISTRY.get_language("erlang")
    } else if shebang.contains("lua") {
        LANGUAGE_REGISTRY.get_language("lua")
    } else {
        None
    }
}

/// Detect language from content patterns
pub fn detect_from_patterns(content: &str) -> Option<&'static LanguageInfo> {
    // Check for unique language patterns

    // Dockerfile
    if content.starts_with("FROM ") || content.contains("\nFROM ") {
        return LANGUAGE_REGISTRY.get_language("dockerfile");
    }

    // SQL
    if content.to_uppercase().starts_with("CREATE TABLE")
        || content.to_uppercase().starts_with("SELECT ")
        || content.to_uppercase().starts_with("INSERT INTO")
    {
        return LANGUAGE_REGISTRY.get_language("sql");
    }

    // YAML (check for common patterns)
    if content.starts_with("---\n") || content.contains(":\n  ") {
        // Could be YAML, but need more checks
        if !content.contains("```") && !content.contains("# ") {
            return LANGUAGE_REGISTRY.get_language("yaml");
        }
    }

    // Markdown
    if content.starts_with("# ") || content.contains("\n## ") || content.contains("```") {
        return LANGUAGE_REGISTRY.get_language("markdown");
    }

    None
}

/// Detect language for ambiguous files (Makefile, README, etc.)
pub fn detect_special_files(filename: &str) -> Option<&'static LanguageInfo> {
    let lower = filename.to_lowercase();

    match lower.as_str() {
        "makefile" | "gnumakefile" => LANGUAGE_REGISTRY.get_language("makefile"),
        "dockerfile" => LANGUAGE_REGISTRY.get_language("dockerfile"),
        "jenkinsfile" => LANGUAGE_REGISTRY.get_language("groovy"),
        "gemfile" | "rakefile" => LANGUAGE_REGISTRY.get_language("ruby"),
        "cargo.toml" => LANGUAGE_REGISTRY.get_language("toml"),
        "package.json" | "tsconfig.json" => LANGUAGE_REGISTRY.get_language("json"),
        "mix.exs" => LANGUAGE_REGISTRY.get_language("elixir"),
        _ => None,
    }
}

/// Check if a language can be detected (returns true/false, no confidence scores)
///
/// Strict checking - either the language is supported or it isn't.
pub fn is_detectable(file_path: &Path, content: Option<&str>) -> bool {
    // Check extension
    if LANGUAGE_REGISTRY.detect_language(file_path).is_ok() {
        return true;
    }

    // Check content if provided
    if let Some(file_content) = content {
        if detect_from_content(file_content).is_some() {
            return true;
        }
    }

    // Check special files
    if let Some(filename) = file_path.file_name() {
        if let Some(name) = filename.to_str() {
            if detect_special_files(name).is_some() {
                return true;
            }
        }
    }

    false
}
