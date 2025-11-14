//! File Classification Engine - Identifies vendored, generated, and binary files
//!
//! This module provides classification rules derived from GitHub Linguist's patterns:
//! <https://github.com/github-linguist/linguist/blob/main/lib/linguist/vendor.yml>
//!
//! ## Classification Categories
//!
//! - **Vendored**: Third-party dependencies (`node_modules/`, `vendor/`, etc.)
//! - **Generated**: Auto-generated files (protobuf, graphql, minified, etc.)
//! - **Binary**: Non-text files (images, archives, compiled binaries)
//! - **Documentation**: Auto-generated docs (Sphinx, Doxygen)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use singularity_language_registry::FileClassifier;
//! use std::path::Path;
//!
//! let classifier = FileClassifier::new();
//! let path = Path::new("node_modules/package/index.js");
//!
//! if classifier.is_vendored(path) {
//!     println!("Skip vendored code");
//! }
//! ```

use std::path::Path;

/// File classification result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FileClass {
    /// Regular source code file
    Source,
    /// Vendored/third-party dependency
    Vendored,
    /// Auto-generated file
    Generated,
    /// Binary file (non-text)
    Binary,
    /// Auto-generated documentation
    Documentation,
}

/// File classifier using Linguist patterns
#[derive(Debug, Clone)]
pub struct FileClassifier {
    /// Vendored file path patterns (from Linguist vendor.yml)
    vendored_patterns: Vec<&'static str>,
    /// Generated file extensions
    generated_extensions: Vec<&'static str>,
    /// Binary file extensions
    binary_extensions: Vec<&'static str>,
    /// Documentation tool markers
    documentation_markers: Vec<&'static str>,
}

impl FileClassifier {
    /// Create a new file classifier with Linguist patterns
    #[must_use]
    pub fn new() -> Self {
        Self {
            vendored_patterns: vec![
                // Dependency directories
                "node_modules/",
                "vendor/",
                "vendors/",
                ".yarn/",
                "Pods/",
                "Carthage/Build/",
                "third_party/",
                "dependencies/",
                // IDE/Editor artifacts
                ".vscode/",
                ".idea/",
                ".sublime-project",
                // Build artifacts
                "dist/",
                "build/",
                "target/",
                "_build/",
                // Package lock files
                "package-lock.json",
                "yarn.lock",
                "Cargo.lock",
                "poetry.lock",
                "Gemfile.lock",
                // Gradle/Maven wrappers
                "gradlew",
                "mvnw",
            ],
            generated_extensions: vec![
                ".pb.rs",       // Protobuf (Rust)
                ".pb.go",       // Protobuf (Go)
                ".pb.py",       // Protobuf (Python)
                ".pb2.py",      // Protobuf v2 (Python)
                ".pb.js",       // Protobuf (JS)
                ".designer.cs", // Visual Studio designer
                ".g.ts",        // Angular/GraphQL generated
                ".generated.ts",
                ".generated.js",
                ".nib", // Xcode Interface Builder
                ".xcworkspacedata",
                ".storyboard",
                ".xib",
                ".meta", // Unity3D metadata
            ],
            binary_extensions: vec![
                // Images
                ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".svg", // Archives
                ".zip", ".tar", ".gz", ".rar", ".7z", // Compiled binaries
                ".exe", ".bin", ".so", ".dll", ".dylib", // Documents (binary formats)
                ".pdf", ".docx", ".xlsx", ".pptx", // Audio/Video
                ".mp3", ".mp4", ".wav", ".avi", ".mov",
            ],
            documentation_markers: vec!["doxygen", "sphinx", "jsdoc", "pandoc"],
        }
    }

    /// Check if path is vendored code
    #[must_use]
    pub fn is_vendored(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.vendored_patterns
            .iter()
            .any(|pattern| path_str.contains(pattern))
    }

    /// Check if file is generated
    #[must_use]
    pub fn is_generated(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.generated_extensions
            .iter()
            .any(|pattern| path_str.ends_with(pattern))
    }

    /// Check if file is binary
    #[must_use]
    pub fn is_binary(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.binary_extensions
            .iter()
            .any(|pattern| path_str.ends_with(pattern))
    }

    /// Check if file is documentation
    #[must_use]
    pub fn is_documentation(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.documentation_markers
            .iter()
            .any(|marker| path_str.contains(marker))
    }

    /// Classify a file path
    #[must_use]
    pub fn classify(&self, path: &Path) -> FileClass {
        if self.is_binary(path) {
            FileClass::Binary
        } else if self.is_vendored(path) {
            FileClass::Vendored
        } else if self.is_generated(path) {
            FileClass::Generated
        } else if self.is_documentation(path) {
            FileClass::Documentation
        } else {
            FileClass::Source
        }
    }

    /// Check if file should be analyzed (not vendored, generated, or binary)
    #[must_use]
    pub fn should_analyze(&self, path: &Path) -> bool {
        matches!(self.classify(path), FileClass::Source)
    }
}

impl Default for FileClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(
    clippy::missing_panics_doc,
    reason = "Test module doesn't need panic documentation"
)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_vendored_detection() {
        let classifier = FileClassifier::new();
        assert!(classifier.is_vendored(&PathBuf::from("node_modules/package/index.js")));
        assert!(classifier.is_vendored(&PathBuf::from("vendor/lib/helper.rb")));
        assert!(!classifier.is_vendored(&PathBuf::from("src/main.rs")));
    }

    #[test]
    fn test_generated_detection() {
        let classifier = FileClassifier::new();
        assert!(classifier.is_generated(&PathBuf::from("api.pb.rs")));
        assert!(classifier.is_generated(&PathBuf::from("Component.generated.ts")));
        assert!(!classifier.is_generated(&PathBuf::from("Component.ts")));
    }

    #[test]
    fn test_binary_detection() {
        let classifier = FileClassifier::new();
        assert!(classifier.is_binary(&PathBuf::from("image.png")));
        assert!(classifier.is_binary(&PathBuf::from("archive.zip")));
        assert!(!classifier.is_binary(&PathBuf::from("script.js")));
    }

    #[test]
    fn test_classification() {
        let classifier = FileClassifier::new();
        assert_eq!(
            classifier.classify(&PathBuf::from("node_modules/pkg/index.js")),
            FileClass::Vendored
        );
        assert_eq!(
            classifier.classify(&PathBuf::from("api.pb.rs")),
            FileClass::Generated
        );
        assert_eq!(
            classifier.classify(&PathBuf::from("image.png")),
            FileClass::Binary
        );
        assert_eq!(
            classifier.classify(&PathBuf::from("src/main.rs")),
            FileClass::Source
        );
    }

    #[test]
    fn test_should_analyze() {
        let classifier = FileClassifier::new();
        assert!(classifier.should_analyze(&PathBuf::from("src/main.rs")));
        assert!(!classifier.should_analyze(&PathBuf::from("node_modules/pkg/index.js")));
        assert!(!classifier.should_analyze(&PathBuf::from("api.pb.rs")));
    }
}
