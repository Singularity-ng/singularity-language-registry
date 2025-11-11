//! Singularity Language Registry
//!
//! Production-ready, centralized language definitions and detection for all Singularity engines.
//!
//! ## Overview
//!
//! This crate provides a single source of truth for language information across all
//! Singularity analysis engines (parsing, analysis, code quality, linting).
//!
//! ## Architecture
//!
//! All engines depend on this registry:
//! ```text
//! language_registry (this crate)
//!     ↑ ↑ ↑ ↑ ↑
//!     │ │ │ │ └─ singularity-linting-engine
//!     │ │ │ └─── singularity-code-engine
//!     │ │ └───── singularity-analysis-engine
//!     │ └─────── singularity-parsing-engine
//!     └───────── singularity-code-nif
//! ```
//!
//! ## Features
//!
//! - **18+ languages** with complete metadata
//! - **Fast lookups** via optimized `HashMaps` for extensions, aliases, MIME types
//! - **Zero dependencies** on other Singularity crates
//! - **Content detection** via shebang, patterns, and magic bytes
//! - **Family grouping** for language categories (BEAM, Systems, Web, etc.)
//! - **Feature detection** for analysis capabilities per language
//! - **Production ready** with comprehensive tests and documentation
//!
//! ## Usage
//!
//! ```rust
//! use singularity_language_registry::{detect_language, get_language, LanguageStats};
//! use std::path::Path;
//!
//! // Detect language from file path
//! let path = Path::new("example.rs");
//! if let Ok(lang) = detect_language(path) {
//!     println!("Language: {}", lang.name);
//!     println!("Family: {:?}", lang.family);
//! }
//!
//! // Get language by ID
//! if let Some(lang) = get_language("elixir") {
//!     println!("Extensions: {:?}", lang.extensions);
//! }
//!
//! // Get statistics
//! let stats = LanguageStats::calculate();
//! println!("Total languages: {}", stats.total_languages);
//! ```

pub mod detection;
pub mod metadata;
pub mod registry;
pub mod utils;

// Core registry exports
pub use registry::{LanguageInfo, LanguageRegistry, PatternSignatures, LANGUAGE_REGISTRY};

// Convenience functions for direct access
pub use registry::{
    ast_grep_supported_languages, detect_language, get_language, get_language_by_alias,
    get_language_by_mime_type, rca_supported_languages, supported_languages,
};

// Detection utilities
pub use detection::{
    detect_from_content, detect_from_patterns, detect_from_shebang, detect_special_files,
    is_detectable,
};

// Utility functions
pub use utils::{
    file_patterns, languages_by_families, recommended_linters, same_family, supports_feature,
    AnalysisFeature, LanguageStats,
};

// Metadata validation and reporting
pub use metadata::{
    generate_metadata_report, get_known_support, validate_metadata, CapabilityMismatch,
    MetadataSource, MetadataValidation,
};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
