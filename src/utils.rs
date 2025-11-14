//! Utility functions for language operations

use crate::registry::{LanguageInfo, LANGUAGE_REGISTRY};
use std::collections::HashMap;
use std::sync::atomic::Ordering;

/// Get all languages grouped by family
pub fn languages_by_families() -> HashMap<String, Vec<&'static LanguageInfo>> {
    let mut families: HashMap<String, Vec<&'static LanguageInfo>> = HashMap::new();

    for lang in LANGUAGE_REGISTRY.supported_languages() {
        if let Some(ref family) = lang.family {
            families.entry(family.clone()).or_default().push(lang);
        } else {
            families.entry("Other".to_owned()).or_default().push(lang);
        }
    }

    families
}

/// Get language statistics
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct LanguageStats {
    pub total_languages: usize,
    pub rca_supported: usize,
    pub ast_grep_supported: usize,
    pub compiled_languages: usize,
    pub interpreted_languages: usize,
    pub families: usize,
}

impl LanguageStats {
    pub fn calculate() -> Self {
        let all = LANGUAGE_REGISTRY.supported_languages();
        let families = languages_by_families();

        Self {
            total_languages: all.len(),
            rca_supported: all
                .iter()
                .filter(|l| l.rca_supported.load(Ordering::Relaxed))
                .count(),
            ast_grep_supported: all
                .iter()
                .filter(|l| l.ast_grep_supported.load(Ordering::Relaxed))
                .count(),
            compiled_languages: all.iter().filter(|l| l.is_compiled).count(),
            interpreted_languages: all.iter().filter(|l| !l.is_compiled).count(),
            families: families.len(),
        }
    }
}

/// Check if two languages are in the same family
pub fn same_family(lang1: &str, lang2: &str) -> bool {
    let Some(l1) = LANGUAGE_REGISTRY.get_language(lang1) else {
        return false;
    };

    let Some(l2) = LANGUAGE_REGISTRY.get_language(lang2) else {
        return false;
    };

    match (l1.family.as_ref(), l2.family.as_ref()) {
        (Some(f1), Some(f2)) => f1 == f2,
        _ => false,
    }
}

/// Get recommended linters for a language
#[must_use]
pub fn recommended_linters(language: &str) -> Vec<&'static str> {
    match language {
        "rust" => vec!["clippy", "rustfmt"],
        "javascript" | "typescript" => vec!["eslint", "prettier"],
        "python" => vec!["pylint", "black", "mypy"],
        "go" => vec!["golangci-lint", "gofmt"],
        "elixir" => vec!["credo", "mix format"],
        "erlang" => vec!["dialyzer"],
        "java" => vec!["spotbugs", "checkstyle"],
        "c" | "cpp" => vec!["clang-tidy", "cppcheck"],
        "csharp" => vec!["roslyn", "sonarqube"],
        "ruby" => vec!["rubocop"],
        _ => vec![],
    }
}

/// Get common file patterns for a language (beyond extensions)
#[must_use]
pub fn file_patterns(language: &str) -> Vec<&'static str> {
    match language {
        "rust" => vec!["Cargo.toml", "Cargo.lock", "build.rs"],
        "javascript" | "typescript" => vec!["package.json", "tsconfig.json", "webpack.config.js"],
        "python" => vec!["requirements.txt", "setup.py", "pyproject.toml", "Pipfile"],
        "go" => vec!["go.mod", "go.sum"],
        "elixir" => vec!["mix.exs", "mix.lock"],
        "erlang" => vec!["rebar.config", "erlang.mk"],
        "java" => vec!["pom.xml", "build.gradle"],
        "ruby" => vec!["Gemfile", "Gemfile.lock", "Rakefile"],
        _ => vec![],
    }
}

/// Check if a language supports a specific analysis feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnalysisFeature {
    RCA,
    ASTGrep,
    TreeSitter,
    Complexity,
    Security,
    Performance,
}

pub fn supports_feature(language: &str, feature: AnalysisFeature) -> bool {
    let Some(lang) = LANGUAGE_REGISTRY.get_language(language) else {
        return false;
    };

    match feature {
        AnalysisFeature::RCA => lang.rca_supported.load(Ordering::Relaxed),
        AnalysisFeature::ASTGrep => lang.ast_grep_supported.load(Ordering::Relaxed),
        AnalysisFeature::TreeSitter => lang.tree_sitter_language.is_some(),
        AnalysisFeature::Complexity => {
            // Most compiled languages support complexity analysis
            lang.is_compiled || matches!(language, "python" | "javascript" | "typescript")
        }
        AnalysisFeature::Security => {
            // Languages commonly used in web/system development
            matches!(
                language,
                "rust"
                    | "go"
                    | "python"
                    | "javascript"
                    | "typescript"
                    | "java"
                    | "csharp"
                    | "c"
                    | "cpp"
                    | "ruby"
                    | "elixir"
            )
        }
        AnalysisFeature::Performance => {
            // Compiled languages + performance-critical interpreted ones
            lang.is_compiled || matches!(language, "python" | "javascript" | "typescript")
        }
    }
}
