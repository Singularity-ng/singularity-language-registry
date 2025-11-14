//! Language Registry - Centralized language management for parser engine
//!
//! This module provides a comprehensive language registry that serves as the single
//! source of truth for all language-related information across the parser engine.
//!
//! ## Features
//!
//! - **Single Source of Truth**: All language information in one place
//! - **Consistent Naming**: Standardized language identifiers across all components
//! - **Capability Tracking**: Explicit tracking of what each language supports
//! - **Performance Optimized**: Pre-built maps for fast lookups
//! - **Extensible**: Easy to add new languages and capabilities

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::LazyLock;

/// Language-level pattern signatures (syntax/keywords only, NOT libraries!)
///
/// **IMPORTANT**: This should ONLY contain language syntax features, NOT libraries/frameworks.
/// Libraries and frameworks (kafka, reqwest, NATS, etc.) should come from `CentralCloud` patterns.
///
/// Examples:
/// - ✅ Language features: `Result<`, `async`, `await`, `?`, `try:`, `catch`
/// - ❌ Libraries: `kafka`, `reqwest`, `express` (these go in `CentralCloud`!)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PatternSignatures {
    /// Error handling SYNTAX (language keywords, not libraries)
    /// Examples: `Result<`, `?`, `unwrap` for Rust, `try:`, `except` for Python
    pub error_handling_syntax: Vec<String>,
    /// Async/concurrency SYNTAX (language keywords, not libraries)
    /// Examples: `async`, `await` for Rust/JS, `spawn`, `Task` for Elixir
    pub async_syntax: Vec<String>,
    /// Testing SYNTAX (language built-ins, not frameworks)
    /// Examples: `#[test]`, `assert!` for Rust, `deftest` for Elixir
    pub testing_syntax: Vec<String>,
    /// Pattern matching SYNTAX
    /// Examples: `match`, `case` for Rust, `case`, `when` for Elixir
    pub pattern_matching_syntax: Vec<String>,
    /// Module/import SYNTAX
    /// Examples: `use`, `mod` for Rust, `import`, `alias` for Elixir
    pub module_syntax: Vec<String>,
}

/// Snapshot representation of language info used when importing from JSON
/// exported from GitHub Linguist. This mirrors `LanguageInfo` but uses
/// plain types (no atomics) for serialization/deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "Boolean flags match LanguageInfo structure for snapshot serialization"
)]
#[non_exhaustive]
pub struct LanguageInfoSnapshot {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub tree_sitter_language: Option<String>,
    #[serde(default)]
    pub rca_supported: bool,
    #[serde(default)]
    pub ast_grep_supported: bool,
    #[serde(default)]
    pub mime_types: Vec<String>,
    pub family: Option<String>,
    #[serde(default)]
    pub is_compiled: bool,
    #[serde(default = "String::new")]
    pub language_type: String,
    #[serde(default)]
    pub pattern_signatures: PatternSignatures,
}

/// Comprehensive language information
///
/// This struct represents a programming language in the Singularity registry.
/// The registry is derived from GitHub Linguist's authoritative language list,
/// ensuring consistency across the ecosystem.
///
/// ## Source of Truth
/// Languages are sourced from <https://github.com/github-linguist/linguist>
/// and tracked by Renovate for automatic updates.
#[derive(Debug, Serialize, Deserialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "Boolean flags for language capabilities are semantically clear and independent"
)]
#[non_exhaustive]
pub struct LanguageInfo {
    /// Unique language identifier (e.g., `"rust"`, `"elixir"`)
    /// Derived from GitHub Linguist language names (lowercased)
    pub id: String,
    /// Human-readable language name (e.g., `"Rust"`, `"Elixir"`)
    pub name: String,
    /// File extensions for this language (e.g., `rs`, or `ex`/`exs`)
    /// Source: GitHub Linguist
    pub extensions: Vec<String>,
    /// Alternative names/aliases (e.g., `js`, `javascript`)
    pub aliases: Vec<String>,
    /// Whether this language is supported by Singularity's parsing engine
    /// Default: false (only explicitly supported languages are true)
    pub supported_in_singularity: bool,
    /// Tree-sitter language name (if supported)
    pub tree_sitter_language: Option<String>,
    /// Whether RCA (rust-code-analysis) supports this language
    pub rca_supported: AtomicBool,
    /// Whether AST-Grep supports this language (set at runtime by engines)
    pub ast_grep_supported: AtomicBool,
    /// MIME types for this language
    pub mime_types: Vec<String>,
    /// Language family (e.g., "BEAM", "C-like", "Web")
    pub family: Option<String>,
    /// Whether this is a compiled or interpreted language
    pub is_compiled: bool,
    /// Language type from Linguist: "programming", "markup", "data", "prose"
    pub language_type: String,
    /// Pattern signatures for cross-language pattern detection
    #[serde(default)]
    pub pattern_signatures: PatternSignatures,
    /// Dynamic capability bits controlled by downstream engines
    #[serde(skip)]
    pub capabilities: AtomicU32,
}

/// Explicit capability bits that downstream engines can toggle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum LanguageCapability {
    RCA = 0,
    ASTGrep = 1,
    Linting = 2,
    Parsing = 3,
    CodeEngine = 4,
}

impl LanguageCapability {
    /// Bitmask for a capability.
    #[must_use]
    pub const fn bit(self) -> u32 {
        #[allow(clippy::as_conversions, reason = "Safe cast from repr(u8) enum to u8")]
        {
            1 << (self as u8)
        }
    }
}

impl LanguageInfo {
    /// Check if a capability bit is enabled.
    #[must_use]
    pub fn has_capability(&self, capability: LanguageCapability) -> bool {
        (self.capabilities.load(Ordering::Relaxed) & capability.bit()) != 0
    }

    /// Enable or disable a capability.
    pub fn set_capability(&self, capability: LanguageCapability, enabled: bool) {
        if enabled {
            #[allow(
                clippy::let_underscore_untyped,
                reason = "Return value intentionally ignored"
            )]
            let _ = self
                .capabilities
                .fetch_or(capability.bit(), Ordering::Relaxed);
        } else {
            #[allow(
                clippy::let_underscore_untyped,
                reason = "Return value intentionally ignored"
            )]
            let _ = self
                .capabilities
                .fetch_and(!capability.bit(), Ordering::Relaxed);
        }

        match capability {
            LanguageCapability::RCA => {
                self.rca_supported.store(enabled, Ordering::Relaxed);
            }
            LanguageCapability::ASTGrep => {
                self.ast_grep_supported.store(enabled, Ordering::Relaxed);
            }
            LanguageCapability::Linting
            | LanguageCapability::Parsing
            | LanguageCapability::CodeEngine => {
                // These capabilities are tracked via the bit flags only
            }
        }
    }
}

/// Central language registry with optimized lookups
#[derive(Debug)]
pub struct LanguageRegistry {
    /// Primary language storage by ID
    languages: HashMap<String, LanguageInfo>,
    /// Fast extension -> language ID mapping
    extension_map: HashMap<String, String>,
    /// Fast alias -> language ID mapping
    alias_map: HashMap<String, String>,
    /// Fast MIME type -> language ID mapping
    mime_map: HashMap<String, String>,
}

impl LanguageRegistry {
    /// Create a new language registry with all supported languages
    ///
    /// # Panics
    ///
    /// Panics if `SINGULARITY_LANGUAGE_SNAPSHOT` environment variable is not set,
    /// if the snapshot file does not exist, if the file cannot be read, or if the
    /// JSON content cannot be parsed. These are intentional release blockers to
    /// prevent shipping with an invalid language registry.
    #[must_use]
    #[allow(
        clippy::too_many_lines,
        reason = "Registry initialization with snapshot loading requires env var checks and error handling"
    )]
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
            extension_map: HashMap::new(),
            alias_map: HashMap::new(),
            mime_map: HashMap::new(),
        };

        // In tests we keep the built-in registration for convenience. In
        // normal builds/releases we require an externally-generated JSON
        // snapshot (exported from GitHub Linguist) to be provided via the
        // SINGULARITY_LANGUAGE_SNAPSHOT env var. If the snapshot is missing or
        // cannot be parsed the process will panic to prevent releasing with an
        // incorrect core list.
        if cfg!(test) {
            registry.register_all_languages();
            return registry;
        }

        #[allow(
            clippy::panic,
            reason = "SINGULARITY_LANGUAGE_SNAPSHOT must be set to a valid languages JSON manifest path before initializing the registry"
        )]
        let snapshot_path = env::var("SINGULARITY_LANGUAGE_SNAPSHOT").unwrap_or_else(|_| {
            panic!("SINGULARITY_LANGUAGE_SNAPSHOT is not set. Provide a JSON snapshot exported from GitHub Linguist and set the env var to its path before building/releasing.");
        });

        let p = Path::new(&snapshot_path);
        #[allow(
            clippy::manual_assert,
            reason = "Panic messages are informative for release blocker"
        )]
        if !p.exists() {
            #[allow(
                clippy::panic,
                reason = "Intentional panic when snapshot is missing - release blocker"
            )]
            {
                panic!("Language snapshot file not found at {snapshot_path}");
            }
        }

        #[allow(
            clippy::panic,
            reason = "Intentional panic on snapshot read failure - release blocker"
        )]
        let contents = fs::read_to_string(p).unwrap_or_else(|e| {
            panic!("Failed to read language snapshot {snapshot_path}: {e}");
        });

        #[allow(
            clippy::panic,
            reason = "Intentional panic on snapshot JSON parse failure - release blocker"
        )]
        let snapshots: Vec<LanguageInfoSnapshot> =
            serde_json::from_str(&contents).unwrap_or_else(|e| {
                panic!("Failed to parse language snapshot {snapshot_path}: {e}");
            });

        for snap in snapshots {
            registry.register_language(LanguageInfo {
                id: snap.id,
                name: snap.name,
                extensions: snap.extensions,
                aliases: snap.aliases,
                supported_in_singularity: false,
                tree_sitter_language: snap.tree_sitter_language,
                rca_supported: AtomicBool::new(snap.rca_supported),
                ast_grep_supported: AtomicBool::new(snap.ast_grep_supported),
                mime_types: snap.mime_types,
                family: snap.family,
                is_compiled: snap.is_compiled,
                language_type: snap.language_type,
                pattern_signatures: snap.pattern_signatures,
                capabilities: AtomicU32::new(0),
            });
        }

        registry
    }

    /// Register all supported languages
    ///
    /// # Panics
    ///
    /// Panics if the test fixture file at `tests/fixtures/builtin_snapshot.json`
    /// cannot be read or if its JSON content cannot be parsed.
    #[allow(
        clippy::too_many_lines,
        reason = "Language registration data is necessarily large; splitting would reduce readability"
    )]
    #[allow(
        dead_code,
        reason = "Fallback language registration; may be used in testing or as a reference"
    )]
    #[allow(
        clippy::panic,
        reason = "Test fixture loading function - panics are expected on invalid fixtures"
    )]
    fn register_all_languages(&mut self) {
        // For tests we load a fixture file with a canonical snapshot. This
        // keeps the source file small and ensures test runs are reproducible
        // without requiring the full snapshot env var.
        let fixture_path = Path::new("tests/fixtures/builtin_snapshot.json");
        let contents = fs::read_to_string(fixture_path).unwrap_or_else(|e| {
            panic!(
                "Failed to read test fixture {}: {}",
                fixture_path.display(),
                e
            )
        });

        let snapshots: Vec<LanguageInfoSnapshot> =
            serde_json::from_str(&contents).unwrap_or_else(|e| {
                panic!(
                    "Failed to parse test fixture {}: {}",
                    fixture_path.display(),
                    e
                )
            });

        for snap in snapshots {
            self.register_language(LanguageInfo {
                id: snap.id,
                name: snap.name,
                extensions: snap.extensions,
                aliases: snap.aliases,
                supported_in_singularity: true,
                tree_sitter_language: snap.tree_sitter_language,
                rca_supported: AtomicBool::new(snap.rca_supported),
                ast_grep_supported: AtomicBool::new(snap.ast_grep_supported),
                mime_types: snap.mime_types,
                family: snap.family,
                is_compiled: snap.is_compiled,
                language_type: snap.language_type,
                pattern_signatures: snap.pattern_signatures,
                capabilities: AtomicU32::new(0),
            });
        }
    }

    /// Register a single language
    #[allow(
        clippy::indexing_slicing,
        reason = "Indexing is safe here - we just inserted the language with this ID"
    )]
    fn register_language(&mut self, language: LanguageInfo) {
        let id = language.id.clone();

        // Store the language (we don't care about previous value, if any)
        let _prev = self.languages.insert(id.clone(), language);

        // Build extension map
        for ext in &self.languages[&id].extensions {
            let _prev = self.extension_map.insert(ext.clone(), id.clone());
        }

        // Build alias map
        for alias in &self.languages[&id].aliases {
            let _prev = self.alias_map.insert(alias.clone(), id.clone());
        }

        // Build MIME type map
        for mime_type in &self.languages[&id].mime_types {
            let _prev = self.mime_map.insert(mime_type.clone(), id.clone());
        }
    }

    /// Detect language from file path
    /// # Errors
    ///
    /// Returns an error when the file extension is missing or unsupported.
    pub fn detect_language(&self, file_path: &Path) -> Result<&LanguageInfo> {
        // Strict detection - no fallback
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Cannot detect language: no file extension for {}",
                    file_path.display()
                )
            })?;

        // Fail if extension not supported
        let language_id = self.extension_map.get(extension).ok_or_else(|| {
            anyhow::anyhow!("Language not supported: unknown extension '.{extension}'")
        })?;

        // Get language (internal consistency check)
        self.languages.get(language_id).ok_or_else(|| {
            anyhow::anyhow!("Internal error: language '{language_id}' missing from registry")
        })
    }

    /// Get language by ID
    #[must_use]
    pub fn get_language(&self, id: &str) -> Option<&LanguageInfo> {
        self.languages.get(id)
    }

    /// Get language by alias
    #[must_use]
    pub fn get_language_by_alias(&self, alias: &str) -> Option<&LanguageInfo> {
        let id = self.alias_map.get(alias)?;
        self.languages.get(id)
    }

    /// Get language by MIME type
    #[must_use]
    pub fn get_language_by_mime_type(&self, mime_type: &str) -> Option<&LanguageInfo> {
        let id = self.mime_map.get(mime_type)?;
        self.languages.get(id)
    }

    /// Get all supported languages
    #[must_use]
    pub fn supported_languages(&self) -> Vec<&LanguageInfo> {
        self.languages.values().collect()
    }

    /// Get languages that support RCA analysis
    #[must_use]
    pub fn rca_supported_languages(&self) -> Vec<&LanguageInfo> {
        self.languages
            .values()
            .filter(|lang| lang.rca_supported.load(Ordering::Relaxed))
            .collect()
    }

    /// Get languages that support AST-Grep
    #[must_use]
    pub fn ast_grep_supported_languages(&self) -> Vec<&LanguageInfo> {
        self.languages
            .values()
            .filter(|lang| lang.ast_grep_supported.load(Ordering::Relaxed))
            .collect()
    }

    /// Register AST-Grep capabilities from downstream engine
    ///
    /// # Errors
    ///
    /// Returns an error if any of the specified languages are not found in the registry.
    pub fn register_ast_grep_capabilities(
        &self,
        supported_languages: &[&str],
    ) -> Result<(), String> {
        // Reset all
        for language in self.languages.values() {
            language.set_capability(LanguageCapability::ASTGrep, false);
        }

        for &language_id in supported_languages {
            self.set_language_capability(language_id, LanguageCapability::ASTGrep, true)?;
        }

        Ok(())
    }

    /// Register which languages have tree-sitter grammars available at runtime.
    /// This toggles the Parsing capability bit but leaves the static
    /// `tree_sitter_language` mapping intact.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the specified languages are not found in the registry.
    pub fn register_tree_sitter_availability(
        &self,
        available_languages: &[&str],
    ) -> Result<(), String> {
        // Disable parsing capability for all
        for language in self.languages.values() {
            language.set_capability(LanguageCapability::Parsing, false);
        }

        for &language_id in available_languages {
            self.set_language_capability(language_id, LanguageCapability::Parsing, true)?;
        }

        Ok(())
    }

    /// Get languages by family
    #[must_use]
    pub fn languages_by_family(&self, family: &str) -> Vec<&LanguageInfo> {
        self.languages
            .values()
            .filter(|lang| lang.family.as_ref().is_some_and(|f| f == family))
            .collect()
    }

    /// Get all language IDs
    #[must_use]
    pub fn language_ids(&self) -> Vec<&String> {
        self.languages.keys().collect()
    }

    /// Get all file extensions
    #[must_use]
    pub fn all_extensions(&self) -> Vec<&String> {
        self.extension_map.keys().collect()
    }

    /// Check if language is supported
    #[must_use]
    pub fn is_supported(&self, id: &str) -> bool {
        self.languages.contains_key(id)
    }

    /// Check if file extension is supported
    #[must_use]
    pub fn is_extension_supported(&self, extension: &str) -> bool {
        self.extension_map.contains_key(extension)
    }

    /// Get language count
    #[must_use]
    pub fn language_count(&self) -> usize {
        self.languages.len()
    }

    /// Set RCA support for a language (called by analysis engine)
    ///
    /// # Errors
    ///
    /// Returns an error if the language is not found in the registry.
    pub fn set_rca_support(&self, language_id: &str, supported: bool) -> Result<(), String> {
        self.set_language_capability(language_id, LanguageCapability::RCA, supported)
    }

    /// Register RCA capabilities from analysis engine
    ///
    /// This method should be called by the analysis engine during initialization
    /// to register which languages it supports for RCA analysis.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the specified languages are not found.
    pub fn register_rca_capabilities(&self, supported_languages: &[&str]) -> Result<(), String> {
        self.register_capability_support(LanguageCapability::RCA, supported_languages)
    }

    /// Enable or disable a specific capability across languages.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the specified languages are not found in the registry.
    pub fn register_capability_support(
        &self,
        capability: LanguageCapability,
        supported_languages: &[&str],
    ) -> Result<(), String> {
        for language in self.languages.values() {
            language.set_capability(capability, false);
        }

        for &language_id in supported_languages {
            self.set_language_capability(language_id, capability, true)?;
        }

        Ok(())
    }

    /// Enable/disable a capability for a single language.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified language is not found in the registry.
    #[allow(
        clippy::too_many_arguments,
        reason = "language_id, capability, and enabled are semantically clear parameters that represent distinct concepts"
    )]
    pub fn set_language_capability(
        &self,
        language_id: &str,
        capability: LanguageCapability,
        enabled: bool,
    ) -> Result<(), String> {
        let language = self
            .languages
            .get(language_id)
            .ok_or_else(|| format!("Language '{language_id}' not found in registry"))?;

        language.set_capability(capability, enabled);
        Ok(())
    }

    /// Get languages that expose a capability.
    #[must_use]
    pub fn languages_with_capability(&self, capability: LanguageCapability) -> Vec<&LanguageInfo> {
        self.languages
            .values()
            .filter(|lang| lang.has_capability(capability))
            .collect()
    }

    /// Get mutable reference to language info for advanced operations
    ///
    /// # Errors
    ///
    /// Returns an error if the language is not found.
    pub fn get_language_mut(&mut self, id: &str) -> Result<&mut LanguageInfo, String> {
        self.languages
            .get_mut(id)
            .ok_or_else(|| format!("Language '{id}' not found"))
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global language registry instance
pub static LANGUAGE_REGISTRY: LazyLock<LanguageRegistry> = LazyLock::new(LanguageRegistry::new);

/// Convenience functions for global registry
///
/// # Errors
///
/// Returns an error when the file extension is missing or unsupported.
pub fn detect_language(file_path: &Path) -> Result<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.detect_language(file_path)
}

#[must_use]
pub fn get_language(id: &str) -> Option<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.get_language(id)
}

#[must_use]
pub fn get_language_by_alias(alias: &str) -> Option<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.get_language_by_alias(alias)
}

#[must_use]
pub fn supported_languages() -> Vec<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.supported_languages()
}

#[must_use]
pub fn rca_supported_languages() -> Vec<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.rca_supported_languages()
}

#[must_use]
pub fn ast_grep_supported_languages() -> Vec<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.ast_grep_supported_languages()
}

#[must_use]
pub fn get_language_by_mime_type(mime_type: &str) -> Option<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.get_language_by_mime_type(mime_type)
}

/// Register RCA (Rust Code Analysis) capabilities for supported languages.
///
/// This function should be called by the analysis engine during initialization
/// to mark which languages it supports for RCA analysis.
///
/// # Errors
///
/// Returns an error if any of the specified languages are not found.
pub fn register_rca_capabilities(supported_languages: &[&str]) -> Result<(), String> {
    LANGUAGE_REGISTRY.register_capability_support(LanguageCapability::RCA, supported_languages)
}

/// Register AST-Grep capabilities from downstream engine.
///
/// # Errors
///
/// Returns an error if any of the specified languages are not found in the registry.
pub fn register_ast_grep_capabilities(supported_languages: &[&str]) -> Result<(), String> {
    LANGUAGE_REGISTRY.register_ast_grep_capabilities(supported_languages)
}

/// Register tree-sitter availability (parsing) for languages available at runtime.
///
/// # Errors
///
/// Returns an error if any of the specified languages are not found in the registry.
pub fn register_tree_sitter_availability(available_languages: &[&str]) -> Result<(), String> {
    LANGUAGE_REGISTRY.register_tree_sitter_availability(available_languages)
}

/// Register capability support for multiple languages.
///
/// This function enables a specific capability for the given languages and disables
/// it for all other languages in the registry.
///
/// # Errors
///
/// Returns an error if any of the specified languages are not found in the registry.
pub fn register_capability_support(
    capability: LanguageCapability,
    supported_languages: &[&str],
) -> Result<(), String> {
    LANGUAGE_REGISTRY.register_capability_support(capability, supported_languages)
}

/// Enable or disable a capability for a single language.
///
/// # Errors
///
/// Returns an error if the specified language is not found in the registry.
pub fn set_language_capability(
    language_id: &str,
    capability: LanguageCapability,
    enabled: bool,
) -> Result<(), String> {
    LANGUAGE_REGISTRY.set_language_capability(language_id, capability, enabled)
}

/// Get languages that expose a capability.
#[must_use]
pub fn languages_with_capability(capability: LanguageCapability) -> Vec<&'static LanguageInfo> {
    LANGUAGE_REGISTRY.languages_with_capability(capability)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::missing_panics_doc,
    reason = "Tests are allowed to panic on unexpected conditions for clearer test failures"
)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_language_detection() {
        // Test Elixir detection
        let elixir_path = Path::new("test.ex");
        let language = detect_language(elixir_path).unwrap();
        assert_eq!(language.id, "elixir");
        assert_eq!(language.name, "Elixir");
        assert!(language.extensions.contains(&"ex".to_owned()));
        assert!(language.extensions.contains(&"exs".to_owned()));
        assert!(!language.rca_supported.load(Ordering::Relaxed));
        assert!(language.ast_grep_supported.load(Ordering::Relaxed));

        // Test Rust detection
        let rust_path = Path::new("test.rs");
        let language = detect_language(rust_path).unwrap();
        assert_eq!(language.id, "rust");
        assert_eq!(language.name, "Rust");
        assert!(!language.rca_supported.load(Ordering::Relaxed));
        assert!(language.ast_grep_supported.load(Ordering::Relaxed));

        // Test JavaScript detection
        let js_path = Path::new("test.js");
        let language = detect_language(js_path).unwrap();
        assert_eq!(language.id, "javascript");
        assert_eq!(language.name, "JavaScript");
        assert!(language.aliases.contains(&"js".to_owned()));
    }

    #[test]
    fn test_language_lookup() {
        // Test by ID
        let language = get_language("elixir").unwrap();
        assert_eq!(language.name, "Elixir");

        // Test by alias
        let language = get_language_by_alias("js").unwrap();
        assert_eq!(language.id, "javascript");

        // Test non-existent language
        assert!(get_language("nonexistent").is_none());
    }

    #[test]
    fn test_capability_registration() {
        let capability = LanguageCapability::Linting;
        let lang = get_language("rust").unwrap();
        assert!(!lang.has_capability(capability));

        set_language_capability("rust", capability, true).unwrap();
        let rust = get_language("rust").unwrap();
        assert!(rust.has_capability(capability));
        assert!(languages_with_capability(capability)
            .iter()
            .any(|language| language.id == "rust"));

        set_language_capability("rust", capability, false).unwrap();
        let rust = get_language("rust").unwrap();
        assert!(!rust.has_capability(capability));
    }

    #[test]
    fn test_register_ast_grep_and_parsing_availability() {
        // Initially AST-Grep should be enabled for languages in the test registry
        let elixir = get_language("elixir").unwrap();
        assert!(elixir.ast_grep_supported.load(Ordering::Relaxed));

        // Disable AST-Grep for elixir via registration
        register_ast_grep_capabilities(&["rust", "javascript"]).unwrap();
        let elixir = get_language("elixir").unwrap();
        assert!(!elixir.ast_grep_supported.load(Ordering::Relaxed));

        // Register tree-sitter availability: only rust and js
        register_tree_sitter_availability(&["rust", "javascript"]).unwrap();
        let rust = get_language("rust").unwrap();
        assert!(rust.has_capability(LanguageCapability::Parsing));
        let elixir = get_language("elixir").unwrap();
        assert!(!elixir.has_capability(LanguageCapability::Parsing));
    }

    #[test]
    fn test_supported_languages() {
        let languages = supported_languages();
        assert!(languages.len() > 10); // Should have many languages
        assert!(languages.iter().any(|lang| lang.id == "elixir"));
        assert!(languages.iter().any(|lang| lang.id == "rust"));
        assert!(languages.iter().any(|lang| lang.id == "javascript"));
    }

    #[test]
    fn test_rca_supported_languages() {
        let rca_languages = rca_supported_languages();
        let rca_ids: Vec<&str> = rca_languages.iter().map(|lang| lang.id.as_str()).collect();

        // RCA is no longer supported by any languages in the parsing engine
        assert!(rca_ids.is_empty());

        // RCA should NOT support BEAM languages
        assert!(!rca_ids.contains(&"elixir"));
        assert!(!rca_ids.contains(&"erlang"));
        assert!(!rca_ids.contains(&"gleam"));
    }

    #[test]
    fn test_ast_grep_supported_languages() {
        let ast_grep_languages = ast_grep_supported_languages();
        let ast_grep_ids: Vec<&str> = ast_grep_languages
            .iter()
            .map(|lang| lang.id.as_str())
            .collect();

        // AST-Grep should support all languages
        assert!(ast_grep_ids.contains(&"elixir"));
        assert!(ast_grep_ids.contains(&"rust"));
        assert!(ast_grep_ids.contains(&"javascript"));
        assert!(ast_grep_ids.contains(&"python"));
        assert!(ast_grep_ids.contains(&"markdown"));
        assert!(ast_grep_ids.contains(&"yaml"));
        assert!(ast_grep_ids.contains(&"json"));
    }

    #[test]
    fn test_language_families() {
        let beam_languages = LANGUAGE_REGISTRY.languages_by_family("BEAM");
        let beam_ids: Vec<&str> = beam_languages.iter().map(|lang| lang.id.as_str()).collect();

        assert!(beam_ids.contains(&"elixir"));
        assert!(beam_ids.contains(&"erlang"));
        assert!(beam_ids.contains(&"gleam"));
        assert_eq!(beam_ids.len(), 3);
    }

    #[test]
    fn test_extension_mapping() {
        assert!(LANGUAGE_REGISTRY.is_extension_supported("rs"));
        assert!(LANGUAGE_REGISTRY.is_extension_supported("ex"));
        assert!(LANGUAGE_REGISTRY.is_extension_supported("js"));
        assert!(LANGUAGE_REGISTRY.is_extension_supported("py"));
        assert!(!LANGUAGE_REGISTRY.is_extension_supported("xyz"));
    }

    #[test]
    fn test_mime_type_detection() {
        let language = get_language_by_mime_type("text/x-rust").unwrap();
        assert_eq!(language.id, "rust");

        let language = get_language_by_mime_type("text/x-elixir").unwrap();
        assert_eq!(language.id, "elixir");

        let language = get_language_by_mime_type("application/json").unwrap();
        assert_eq!(language.id, "json");
    }
}
