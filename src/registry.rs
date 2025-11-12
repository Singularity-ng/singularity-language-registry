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
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
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
    /// Whether AST-Grep supports this language
    pub ast_grep_supported: bool,
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
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
            extension_map: HashMap::new(),
            alias_map: HashMap::new(),
            mime_map: HashMap::new(),
        };

        // Register all supported languages
        registry.register_all_languages();
        registry
    }

    /// Register all supported languages
    #[allow(
        clippy::too_many_lines,
        reason = "Language registration data is necessarily large; splitting would reduce readability"
    )]
    fn register_all_languages(&mut self) {
        // BEAM Languages
        self.register_language(LanguageInfo {
            id: "elixir".to_owned(),
            name: "Elixir".to_owned(),
            extensions: vec!["ex".to_owned(), "exs".to_owned()],
            aliases: vec!["elixir".to_owned()],
            supported_in_singularity: true,
            tree_sitter_language: Some("elixir".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/x-elixir".to_owned(),
                "application/x-elixir".to_owned(),
            ],
            family: Some("BEAM".to_owned()),
            is_compiled: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "erlang".to_owned(),
            name: "Erlang".to_owned(),
            extensions: vec!["erl".to_owned(), "hrl".to_owned()],
            aliases: vec!["erlang".to_owned()],
            supported_in_singularity: true,
            tree_sitter_language: Some("erlang".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/x-erlang".to_owned(),
                "application/x-erlang".to_owned(),
            ],
            family: Some("BEAM".to_owned()),
            is_compiled: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "gleam".to_owned(),
            name: "Gleam".to_owned(),
            extensions: vec!["gleam".to_owned()],
            aliases: vec!["gleam".to_owned()],
            supported_in_singularity: true,
            tree_sitter_language: Some("gleam".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-gleam".to_owned(), "application/x-gleam".to_owned()],
            family: Some("BEAM".to_owned()),
            is_compiled: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Systems Programming Languages
        self.register_language(LanguageInfo {
            id: "rust".to_owned(),
            name: "Rust".to_owned(),
            extensions: vec!["rs".to_owned()],
            aliases: vec!["rust".to_owned()],
            tree_sitter_language: Some("rust".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-rust".to_owned(), "application/x-rust".to_owned()],
            family: Some("Systems".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures {
                // Only language syntax, NOT libraries!
                error_handling_syntax: vec![
                    "Result<".to_owned(),
                    "Option<".to_owned(),
                    "?".to_owned(),
                    "unwrap".to_owned(),
                    "expect".to_owned(),
                ],
                async_syntax: vec!["async".to_owned(), "await".to_owned(), ".await".to_owned()],
                testing_syntax: vec![
                    "#[test]".to_owned(),
                    "assert!".to_owned(),
                    "assert_eq!".to_owned(),
                    "#[cfg(test)]".to_owned(),
                ],
                pattern_matching_syntax: vec![
                    "match".to_owned(),
                    "if let".to_owned(),
                    "while let".to_owned(),
                ],
                module_syntax: vec![
                    "use".to_owned(),
                    "mod".to_owned(),
                    "pub".to_owned(),
                    "crate::".to_owned(),
                ],
            },
        });

        self.register_language(LanguageInfo {
            id: "c".to_owned(),
            name: "C".to_owned(),
            extensions: vec!["c".to_owned(), "h".to_owned()],
            aliases: vec!["c".to_owned()],
            tree_sitter_language: Some("c".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-c".to_owned(), "text/x-csrc".to_owned()],
            family: Some("C-like".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "cpp".to_owned(),
            name: "C++".to_owned(),
            extensions: vec![
                "cpp".to_owned(),
                "cc".to_owned(),
                "cxx".to_owned(),
                "c++".to_owned(),
                "hpp".to_owned(),
            ],
            aliases: vec!["cpp".to_owned(), "c++".to_owned(), "cplusplus".to_owned()],
            tree_sitter_language: Some("cpp".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-c++".to_owned(), "text/x-cpp".to_owned()],
            family: Some("C-like".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Web Technologies
        self.register_language(LanguageInfo {
            id: "javascript".to_owned(),
            name: "JavaScript".to_owned(),
            extensions: vec!["js".to_owned(), "jsx".to_owned()],
            aliases: vec!["javascript".to_owned(), "js".to_owned()],
            tree_sitter_language: Some("javascript".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/javascript".to_owned(),
                "application/javascript".to_owned(),
            ],
            family: Some("Web".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "typescript".to_owned(),
            name: "TypeScript".to_owned(),
            extensions: vec!["ts".to_owned(), "tsx".to_owned()],
            aliases: vec!["typescript".to_owned(), "ts".to_owned()],
            tree_sitter_language: Some("typescript".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/typescript".to_owned(),
                "application/typescript".to_owned(),
            ],
            family: Some("Web".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // High-Level Languages
        self.register_language(LanguageInfo {
            id: "python".to_owned(),
            name: "Python".to_owned(),
            extensions: vec!["py".to_owned(), "pyw".to_owned()],
            aliases: vec!["python".to_owned(), "py".to_owned()],
            tree_sitter_language: Some("python".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/x-python".to_owned(),
                "application/x-python".to_owned(),
            ],
            family: Some("Scripting".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // JVM Languages
        self.register_language(LanguageInfo {
            id: "java".to_owned(),
            name: "Java".to_owned(),
            extensions: vec!["java".to_owned()],
            aliases: vec!["java".to_owned()],
            tree_sitter_language: Some("java".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-java".to_owned(), "application/x-java".to_owned()],
            family: Some("JVM".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Scripting Languages

        self.register_language(LanguageInfo {
            id: "csharp".to_owned(),
            name: "C#".to_owned(),
            extensions: vec!["cs".to_owned()],
            aliases: vec!["csharp".to_owned(), "cs".to_owned(), "c#".to_owned()],
            tree_sitter_language: Some("c_sharp".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec![
                "text/x-csharp".to_owned(),
                "application/x-csharp".to_owned(),
            ],
            family: Some("CLR".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "go".to_owned(),
            name: "Go".to_owned(),
            extensions: vec!["go".to_owned()],
            aliases: vec!["go".to_owned(), "golang".to_owned()],
            tree_sitter_language: Some("go".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-go".to_owned(), "application/x-go".to_owned()],
            family: Some("Systems".to_owned()),
            is_compiled: true,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Scripting Languages
        self.register_language(LanguageInfo {
            id: "lua".to_owned(),
            name: "Lua".to_owned(),
            extensions: vec!["lua".to_owned()],
            aliases: vec!["lua".to_owned()],
            tree_sitter_language: Some("lua".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-lua".to_owned(), "application/x-lua".to_owned()],
            family: Some("Scripting".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "bash".to_owned(),
            name: "Bash".to_owned(),
            extensions: vec!["sh".to_owned(), "bash".to_owned()],
            aliases: vec!["bash".to_owned(), "sh".to_owned(), "shell".to_owned()],
            tree_sitter_language: Some("bash".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-sh".to_owned(), "application/x-sh".to_owned()],
            family: Some("Shell".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Data Formats
        self.register_language(LanguageInfo {
            id: "json".to_owned(),
            name: "JSON".to_owned(),
            extensions: vec!["json".to_owned()],
            aliases: vec!["json".to_owned()],
            tree_sitter_language: Some("json".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["application/json".to_owned()],
            family: Some("Data".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "yaml".to_owned(),
            name: "YAML".to_owned(),
            extensions: vec!["yaml".to_owned(), "yml".to_owned()],
            aliases: vec!["yaml".to_owned(), "yml".to_owned()],
            tree_sitter_language: Some("yaml".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/yaml".to_owned(), "application/x-yaml".to_owned()],
            family: Some("Data".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "toml".to_owned(),
            name: "TOML".to_owned(),
            extensions: vec!["toml".to_owned()],
            aliases: vec!["toml".to_owned()],
            tree_sitter_language: Some("toml".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-toml".to_owned(), "application/toml".to_owned()],
            family: Some("Data".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Documentation
        self.register_language(LanguageInfo {
            id: "markdown".to_owned(),
            name: "Markdown".to_owned(),
            extensions: vec!["md".to_owned(), "markdown".to_owned()],
            aliases: vec!["markdown".to_owned(), "md".to_owned()],
            tree_sitter_language: Some("markdown".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/markdown".to_owned(), "text/x-markdown".to_owned()],
            family: Some("Documentation".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        // Infrastructure
        self.register_language(LanguageInfo {
            id: "dockerfile".to_owned(),
            name: "Dockerfile".to_owned(),
            extensions: vec!["dockerfile".to_owned(), "Dockerfile".to_owned()],
            aliases: vec!["dockerfile".to_owned(), "docker".to_owned()],
            tree_sitter_language: Some("dockerfile".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-dockerfile".to_owned()],
            family: Some("Infrastructure".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });

        self.register_language(LanguageInfo {
            id: "sql".to_owned(),
            name: "SQL".to_owned(),
            extensions: vec!["sql".to_owned()],
            aliases: vec!["sql".to_owned()],
            tree_sitter_language: Some("sql".to_owned()),
            rca_supported: AtomicBool::new(false),
            ast_grep_supported: true,
            mime_types: vec!["text/x-sql".to_owned(), "application/sql".to_owned()],
            family: Some("Database".to_owned()),
            is_compiled: false,
            supported_in_singularity: true,
            language_type: "programming".to_owned(),
            pattern_signatures: PatternSignatures::default(),
        });
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
            .filter(|lang| lang.ast_grep_supported)
            .collect()
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
    pub fn set_rca_support(&mut self, language_id: &str, supported: bool) -> Result<(), String> {
        if let Some(language) = self.languages.get_mut(language_id) {
            language.rca_supported.store(supported, Ordering::Relaxed);
            Ok(())
        } else {
            Err(format!("Language '{language_id}' not found in registry"))
        }
    }

    /// Register RCA capabilities from analysis engine
    ///
    /// This method should be called by the analysis engine during initialization
    /// to register which languages it supports for RCA analysis.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the specified languages are not found.
    pub fn register_rca_capabilities(
        &mut self,
        supported_languages: &[&str],
    ) -> Result<(), String> {
        // First, set all languages to not supported
        for language in self.languages.values_mut() {
            language.rca_supported.store(false, Ordering::Relaxed);
        }

        // Then set the supported ones to true
        for &language_id in supported_languages {
            self.set_rca_support(language_id, true)?;
        }

        Ok(())
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
    // First, set all languages to not supported
    for language in LANGUAGE_REGISTRY.supported_languages() {
        language.rca_supported.store(false, Ordering::Relaxed);
    }

    // Then set the supported ones to true
    for &language_id in supported_languages {
        if let Some(language) = LANGUAGE_REGISTRY.get_language(language_id) {
            language.rca_supported.store(true, Ordering::Relaxed);
        } else {
            return Err(format!("Language '{language_id}' not found in registry"));
        }
    }

    Ok(())
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
        assert!(language.ast_grep_supported);

        // Test Rust detection
        let rust_path = Path::new("test.rs");
        let language = detect_language(rust_path).unwrap();
        assert_eq!(language.id, "rust");
        assert_eq!(language.name, "Rust");
        assert!(!language.rca_supported.load(Ordering::Relaxed));
        assert!(language.ast_grep_supported);

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
