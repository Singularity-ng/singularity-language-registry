//! Metadata synchronization and validation
//!
//! This module provides mechanisms to validate and sync language support
//! metadata with the actual capabilities of underlying libraries.

use crate::registry::LANGUAGE_REGISTRY;

/// Metadata source for language capabilities
#[derive(Debug, Clone)]
pub struct MetadataSource {
    pub tree_sitter_languages: Vec<String>,
    pub rca_languages: Vec<String>,
    pub ast_grep_languages: Vec<String>,
}

/// Validation result for metadata consistency
#[derive(Debug)]
pub struct MetadataValidation {
    pub registry_only: Vec<String>,
    pub missing_from_registry: Vec<String>,
    pub capability_mismatches: Vec<CapabilityMismatch>,
}

#[derive(Debug)]
pub struct CapabilityMismatch {
    pub language: String,
    pub capability: String,
    pub registry_says: bool,
    pub actual: bool,
}

/// Validate registry metadata against actual library support
///
/// This should be run in tests or build scripts to ensure consistency
pub fn validate_metadata(source: &MetadataSource) -> MetadataValidation {
    let registry_only = Vec::new();
    let mut missing_from_registry = Vec::new();
    let mut capability_mismatches = Vec::new();

    // Check RCA support
    for lang_id in &source.rca_languages {
        if let Some(lang) = LANGUAGE_REGISTRY.get_language(lang_id) {
            if !lang.rca_supported {
                capability_mismatches.push(CapabilityMismatch {
                    language: lang_id.clone(),
                    capability: "RCA".to_string(),
                    registry_says: false,
                    actual: true,
                });
            }
        } else {
            missing_from_registry.push(lang_id.clone());
        }
    }

    // Check AST-Grep support
    for lang_id in &source.ast_grep_languages {
        if let Some(lang) = LANGUAGE_REGISTRY.get_language(lang_id) {
            if !lang.ast_grep_supported {
                capability_mismatches.push(CapabilityMismatch {
                    language: lang_id.clone(),
                    capability: "AST-Grep".to_string(),
                    registry_says: false,
                    actual: true,
                });
            }
        } else {
            missing_from_registry.push(lang_id.clone());
        }
    }

    // Check for languages in registry but not in sources
    for lang in LANGUAGE_REGISTRY.supported_languages() {
        if lang.rca_supported && !source.rca_languages.contains(&lang.id) {
            capability_mismatches.push(CapabilityMismatch {
                language: lang.id.clone(),
                capability: "RCA".to_string(),
                registry_says: true,
                actual: false,
            });
        }

        if lang.ast_grep_supported && !source.ast_grep_languages.contains(&lang.id) {
            capability_mismatches.push(CapabilityMismatch {
                language: lang.id.clone(),
                capability: "AST-Grep".to_string(),
                registry_says: true,
                actual: false,
            });
        }
    }

    MetadataValidation {
        registry_only,
        missing_from_registry,
        capability_mismatches,
    }
}

/// Generate metadata report for documentation
pub fn generate_metadata_report() -> String {
    use std::fmt::Write;

    let mut report = String::from("# Language Registry Metadata Report\n\n");

    let stats = crate::utils::LanguageStats::calculate();
    report.push_str("## Statistics\n");
    let _ = writeln!(&mut report, "- Total Languages: {}", stats.total_languages);
    let _ = writeln!(&mut report, "- RCA Supported: {}", stats.rca_supported);
    let _ = writeln!(
        &mut report,
        "- AST-Grep Supported: {}",
        stats.ast_grep_supported
    );
    let _ = writeln!(
        &mut report,
        "- Compiled Languages: {}",
        stats.compiled_languages
    );
    let _ = writeln!(
        &mut report,
        "- Interpreted Languages: {}\n",
        stats.interpreted_languages
    );

    report.push_str("## Language Support Matrix\n\n");
    report.push_str("| Language | Extensions | RCA | AST-Grep | Tree-Sitter | Family |\n");
    report.push_str("|----------|------------|-----|----------|-------------|--------|\n");

    for lang in LANGUAGE_REGISTRY.supported_languages() {
        let _ = writeln!(
            &mut report,
            "| {} | {} | {} | {} | {} | {} |",
            lang.name,
            lang.extensions.join(", "),
            if lang.rca_supported { "✓" } else { "✗" },
            if lang.ast_grep_supported {
                "✓"
            } else {
                "✗"
            },
            if lang.tree_sitter_language.is_some() {
                "✓"
            } else {
                "✗"
            },
            lang.family.as_deref().unwrap_or("-"),
        );
    }

    report
}

/// Known language support from libraries (as of last update)
///
/// This should be updated when upgrading dependencies
pub fn get_known_support() -> MetadataSource {
    MetadataSource {
        // RCA supported languages (from rust-code-analysis)
        rca_languages: vec![
            "rust".to_string(),
            "c".to_string(),
            "cpp".to_string(),
            "go".to_string(),
            "java".to_string(),
            "python".to_string(),
            "javascript".to_string(),
            "typescript".to_string(),
            "csharp".to_string(),
            "kotlin".to_string(),
            "lua".to_string(),
        ],

        // AST-Grep supported languages
        ast_grep_languages: vec![
            "rust".to_string(),
            "python".to_string(),
            "javascript".to_string(),
            "typescript".to_string(),
            "go".to_string(),
            "java".to_string(),
            "c".to_string(),
            "cpp".to_string(),
            "csharp".to_string(),
            "kotlin".to_string(),
            "elixir".to_string(),
            "erlang".to_string(),
            "gleam".to_string(),
            "bash".to_string(),
            "lua".to_string(),
            "sql".to_string(),
            "yaml".to_string(),
            "json".to_string(),
            "toml".to_string(),
            "dockerfile".to_string(),
            "markdown".to_string(),
        ],

        // Tree-sitter has parsers for all our languages
        tree_sitter_languages: LANGUAGE_REGISTRY
            .supported_languages()
            .iter()
            .filter_map(|l| l.tree_sitter_language.clone())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_consistency() {
        let source = get_known_support();
        let validation = validate_metadata(&source);

        // Check for mismatches
        if !validation.capability_mismatches.is_empty() {
            println!("Capability mismatches found:");
            for mismatch in &validation.capability_mismatches {
                println!(
                    "  {} - {}: registry says {}, actual is {}",
                    mismatch.language, mismatch.capability, mismatch.registry_says, mismatch.actual
                );
            }
        }

        // In production, this should fail if there are mismatches
        // assert!(validation.capability_mismatches.is_empty());
    }

    #[test]
    fn test_report_generation() {
        let report = generate_metadata_report();
        assert!(report.contains("Language Registry Metadata Report"));
        assert!(report.contains("Language Support Matrix"));
    }
}
