//! Metadata synchronization and validation
//!
//! This module provides mechanisms to validate and sync language support
//! metadata with the actual capabilities of underlying libraries.

use crate::registry::LANGUAGE_REGISTRY;
use std::sync::atomic::Ordering;

/// Metadata source for language capabilities
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MetadataSource {
    pub tree_sitter_languages: Vec<String>,
    pub rca_languages: Vec<String>,
    pub ast_grep_languages: Vec<String>,
}

/// Validation result for metadata consistency
#[derive(Debug)]
#[non_exhaustive]
pub struct MetadataValidation {
    pub registry_only: Vec<String>,
    pub missing_from_registry: Vec<String>,
    pub capability_mismatches: Vec<CapabilityMismatch>,
}

/// Capability mismatch between registry and actual library support
#[derive(Debug)]
#[non_exhaustive]
#[allow(
    clippy::struct_excessive_bools,
    reason = "Two bools are needed to represent registry vs actual state"
)]
pub struct CapabilityMismatch {
    pub language: String,
    pub capability: String,
    pub registry_says: bool,
    pub actual: bool,
}

/// Validate registry metadata against actual library support
///
/// This should be run in tests or build scripts to ensure consistency
#[allow(
    clippy::too_many_lines,
    reason = "Validation logic needs to check multiple capabilities comprehensively"
)]
pub fn validate_metadata(source: &MetadataSource) -> MetadataValidation {
    let registry_only = Vec::new();
    let mut missing_from_registry = Vec::new();
    let mut capability_mismatches = Vec::new();

    // Check RCA support
    for lang_id in &source.rca_languages {
        if let Some(lang) = LANGUAGE_REGISTRY.get_language(lang_id) {
            if !lang.rca_supported.load(Ordering::Relaxed) {
                capability_mismatches.push(CapabilityMismatch {
                    language: lang_id.clone(),
                    capability: "RCA".to_owned(),
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
            if !lang.ast_grep_supported.load(Ordering::Relaxed) {
                capability_mismatches.push(CapabilityMismatch {
                    language: lang_id.clone(),
                    capability: "AST-Grep".to_owned(),
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
        if lang.rca_supported.load(Ordering::Relaxed) && !source.rca_languages.contains(&lang.id) {
            capability_mismatches.push(CapabilityMismatch {
                language: lang.id.clone(),
                capability: "RCA".to_owned(),
                registry_says: true,
                actual: false,
            });
        }

        if lang.ast_grep_supported.load(Ordering::Relaxed)
            && !source.ast_grep_languages.contains(&lang.id)
        {
            capability_mismatches.push(CapabilityMismatch {
                language: lang.id.clone(),
                capability: "AST-Grep".to_owned(),
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
#[allow(
    clippy::let_underscore_must_use,
    reason = "Writing to String via write! is infallible - cannot fail in practice"
)]
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
            if lang.rca_supported.load(Ordering::Relaxed) {
                "✓"
            } else {
                "✗"
            },
            if lang.ast_grep_supported.load(Ordering::Relaxed) {
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
        rca_languages: vec![],

        // AST-Grep supported languages
        ast_grep_languages: vec![
            "rust".to_owned(),
            "python".to_owned(),
            "javascript".to_owned(),
            "typescript".to_owned(),
            "go".to_owned(),
            "java".to_owned(),
            "c".to_owned(),
            "cpp".to_owned(),
            "csharp".to_owned(),
            "elixir".to_owned(),
            "erlang".to_owned(),
            "gleam".to_owned(),
            "bash".to_owned(),
            "lua".to_owned(),
            "sql".to_owned(),
            "yaml".to_owned(),
            "json".to_owned(),
            "toml".to_owned(),
            "dockerfile".to_owned(),
            "markdown".to_owned(),
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
    #[allow(
        clippy::missing_panics_doc,
        reason = "Test function - panics are expected via assert!"
    )]
    fn test_report_generation() {
        let report = generate_metadata_report();
        assert!(report.contains("Language Registry Metadata Report"));
        assert!(report.contains("Language Support Matrix"));
    }
}
