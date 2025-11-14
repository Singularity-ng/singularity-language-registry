//! Synchronize file classification and language detection patterns from GitHub Linguist.
//!
//! Generates and syncs multiple phases:
//! - Phase 2: File classification (vendor, generated, binary patterns)
//! - Phase 3: Language detection heuristics (ambiguous extension resolution)
//! - Phase 4: Language metadata (extensions, colors, interpreters, editor modes)
//!
//! Run with: `cargo run --bin sync-linguist --features sync-tool`

use anyhow::{Context, Result};
use log::info;
use regex::Regex;
use serde::de::Deserializer;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::{self, Write as _},
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

const VENDOR_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/vendor.yml";
const GENERATED_RB_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/generated.rb";
const HEURISTICS_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/heuristics.yml";
const LANGUAGES_YML_URL: &str =
    "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/languages.yml";

const FILE_CLASSIFIER_PATH: &str = "src/file_classifier_generated.rs";
const HEURISTICS_PATH: &str = "src/heuristics_generated.rs";
const LANGUAGES_METADATA_PATH: &str = "src/languages_metadata_generated.rs";
const LANGUAGES_YML_OUTPUT_PATH: &str = ".github/linguist/languages.yml";

#[derive(Debug, Deserialize)]
struct HeuristicsFile {
    disambiguations: Vec<Disambiguation>,
    #[serde(default)]
    named_patterns: BTreeMap<String, StringList>,
}

#[derive(Debug, Deserialize)]
struct Disambiguation {
    extensions: Vec<String>,
    rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
struct Rule {
    #[serde(deserialize_with = "deserialize_string_vec")]
    language: Vec<String>,
    #[serde(default)]
    pattern: Option<StringList>,
    #[serde(default)]
    named_pattern: Option<String>,
    #[serde(default)]
    negative_pattern: Option<StringList>,
    #[serde(default)]
    and: Vec<RuleCondition>,
}

#[derive(Debug, Deserialize)]
struct RuleCondition {
    #[serde(default)]
    pattern: Option<StringList>,
    #[serde(default)]
    named_pattern: Option<String>,
    #[serde(default)]
    negative_pattern: Option<StringList>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum StringList {
    Single(String),
    Multiple(Vec<String>),
}

impl StringList {
    #[allow(
        clippy::missing_const_for_fn,
        reason = "StringList holds heap data that cannot be manipulated in const contexts"
    )]
    #[allow(
        clippy::pattern_type_mismatch,
        reason = "Matching on &StringList to borrow inner data without cloning"
    )]
    fn as_slice(&self) -> &[String] {
        match self {
            Self::Single(single_value) => std::slice::from_ref(single_value),
            Self::Multiple(values) => values.as_slice(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(
    clippy::struct_field_names,
    reason = "Field names match Linguist's schema"
)]
struct Language {
    #[serde(rename = "type")]
    language_type: String,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    interpreters: Vec<String>,
    #[serde(default)]
    filenames: Vec<String>,
    #[serde(default)]
    ace_mode: Option<String>,
    #[serde(default)]
    tm_scope: Option<String>,
    #[serde(default)]
    language_id: Option<i64>,
    #[serde(default)]
    codemirror_mode: Option<String>,
    #[serde(default)]
    codemirror_mime_type: Option<String>,
    #[serde(default)]
    group: Option<String>,
    #[serde(default)]
    wrap: Option<bool>,
    #[serde(default)]
    fs_name: Option<String>,
}

/// Deserialize either a single string or a list of strings into a `Vec<String>`.
///
/// # Errors
///
/// Returns an error when the input cannot be interpreted as a string or list of strings.
fn deserialize_string_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        Single(String),
        Multiple(Vec<String>),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::Single(single) => Ok(vec![single]),
        StringOrVec::Multiple(values) => Ok(values),
    }
}

/// Fetch the contents of a remote Linguist data file.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or the response body cannot be
/// read before the timeout elapses.
async fn fetch_url(url: &str) -> Result<String> {
    info!("📥 Fetching {url}");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(Duration::from_secs(45))
        .send()
        .await
        .with_context(|| format!("Failed to fetch {url}"))?;

    let status = response.status();
    let content = response
        .text()
        .await
        .context("Failed to read response body")?;

    if !status.is_success() {
        anyhow::bail!("Request to {url} failed with status {status}");
    }

    info!("✅ Fetched {} bytes from {url}", content.len());
    Ok(content)
}

/// Parse Linguist's `vendor.yml` into a sorted set of patterns.
///
/// # Errors
///
/// Returns an error when the YAML payload cannot be deserialized.
fn parse_vendor_yml(content: &str) -> Result<BTreeSet<String>> {
    let patterns: Vec<String> =
        serde_yaml::from_str(content).context("Failed to parse vendor.yml as YAML list")?;
    Ok(patterns.into_iter().collect())
}

/// Parse Linguist's `generated.rb` file and collect quoted patterns.
///
/// # Errors
///
/// Returns an error when the extraction regex cannot be compiled.
fn parse_generated_rb(content: &str) -> Result<HashSet<String>> {
    let quote_pattern =
        Regex::new(r#"['"]([^'"]+)['"]"#).context("Failed to compile generated.rb regex")?;
    let mut patterns = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        for capture in quote_pattern.captures_iter(trimmed) {
            if let Some(matched) = capture.get(1) {
                let candidate = matched.as_str();
                if candidate.len() < 160 {
                    let _was_new = patterns.insert(candidate.to_owned());
                }
            }
        }
    }

    Ok(patterns)
}

/// Parse Linguist's `heuristics.yml` describing disambiguation rules.
///
/// # Errors
///
/// Returns an error when the YAML payload cannot be deserialized.
fn parse_heuristics_yml(content: &str) -> Result<HeuristicsFile> {
    let parsed: HeuristicsFile =
        serde_yaml::from_str(content).context("Failed to deserialize heuristics.yml")?;
    Ok(parsed)
}

fn split_generated_patterns(generated: &HashSet<String>) -> (BTreeSet<String>, BTreeSet<String>) {
    const BINARY_EXTENSIONS: &[&str] = &[
        ".3ds", ".3mf", ".7z", ".aac", ".avi", ".bmp", ".bz2", ".class", ".db", ".dylib", ".dll",
        ".dmg", ".eot", ".exe", ".flac", ".gif", ".gz", ".ico", ".jar", ".jpeg", ".jpg", ".m4a",
        ".mkv", ".mov", ".mp3", ".mp4", ".mpeg", ".mpg", ".ogg", ".ogv", ".pdf", ".png", ".psd",
        ".rar", ".sqlite", ".svg", ".tar", ".tiff", ".ttf", ".wasm", ".webm", ".webp", ".woff",
        ".woff2", ".zip",
    ];

    let mut generated_patterns = BTreeSet::new();
    let mut binary_patterns = BTreeSet::new();

    for pattern in generated {
        if let Some(stripped) = pattern.strip_prefix('.') {
            let lower = format!(".{}", stripped.to_ascii_lowercase());
            if BINARY_EXTENSIONS.contains(&lower.as_str()) {
                let _was_new = binary_patterns.insert(pattern.clone());
                continue;
            }
        }

        let _was_new = generated_patterns.insert(pattern.clone());
    }

    (generated_patterns, binary_patterns)
}

fn escape_rust_string(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Generate the Phase 2 (file classifier) Rust source.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn generate_file_classifier_code(
    vendored: &BTreeSet<String>,
    generated: &BTreeSet<String>,
    binary: &BTreeSet<String>,
) -> Result<String> {
    let mut code = String::new();

    writeln!(
        code,
        "// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY\n\
         // Generated from GitHub Linguist patterns (vendor.yml & generated.rb)\n\
         // Run: cargo run --bin sync-linguist --features sync-tool\n\
         \n\
         //! Auto-generated file classification patterns from GitHub Linguist.\n\
         //! Sources:\n\
         //!   • vendor.yml   → vendored path patterns\n\
         //!   • generated.rb → generated & binary file patterns\n"
    )?;

    writeln!(
        code,
        "/// Vendored code path patterns (from GitHub Linguist vendor.yml)\n\
         pub const VENDORED_PATTERNS_FROM_LINGUIST: &[&str] = &["
    )?;
    for pattern in vendored {
        writeln!(code, "    \"{}\",", escape_rust_string(pattern))?;
    }
    writeln!(code, "]\n")?;

    writeln!(
        code,
        "/// Generated file patterns (from GitHub Linguist generated.rb)\n\
         pub const GENERATED_PATTERNS_FROM_LINGUIST: &[&str] = &["
    )?;
    for pattern in generated {
        writeln!(code, "    \"{}\",", escape_rust_string(pattern))?;
    }
    writeln!(code, "]\n")?;

    writeln!(
        code,
        "/// Binary file extensions detected by GitHub Linguist\n\
         pub const BINARY_PATTERNS_FROM_LINGUIST: &[&str] = &["
    )?;
    for pattern in binary {
        writeln!(code, "    \"{}\",", escape_rust_string(pattern))?;
    }
    writeln!(code, "];")?;

    Ok(code)
}

/// Write a slice literal (e.g., `patterns: &[...]`) to the generated source.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn push_pattern_slice(code: &mut String, label: &str, patterns: &[String]) -> fmt::Result {
    if patterns.is_empty() {
        return writeln!(code, "        {label}: &[],");
    }

    writeln!(code, "        {label}: &[")?;
    for pattern in patterns {
        writeln!(
            code,
            "            \"{}\",",
            escape_rust_string(pattern.as_str())
        )?;
    }
    writeln!(code, "        ],")
}

/// Generate the Phase 3 (heuristics) Rust source.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn generate_heuristics_code(heuristics: &HeuristicsFile) -> Result<String> {
    let mut code = String::new();
    write_heuristics_header(&mut code)?;
    write_named_patterns_section(&mut code, heuristics)?;
    write_disambiguations_section(&mut code, heuristics)?;
    Ok(code)
}

/// Write the documentation banner and helper structs for the heuristics file.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn write_heuristics_header(code: &mut String) -> fmt::Result {
    writeln!(
        code,
        "// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY\n\
         // Generated from GitHub Linguist heuristics.yml\n\
         // Run: cargo run --bin sync-linguist --features sync-tool\n\
         \n\
         //! Auto-generated language detection heuristics sourced from GitHub Linguist.\n\
         //! The data model mirrors Linguist's disambiguation rules so downstream consumers\n\
         //! can evaluate ambiguous extensions without depending on Linguist directly.\n\
         \n\
         #[derive(Debug)]\n\
         pub struct HeuristicRuleCondition {{\n\
             pub patterns: &'static [&'static str],\n\
             pub named_pattern: Option<&'static str>,\n\
             pub negative_patterns: &'static [&'static str],\n\
         }}\n\
         \n\
         #[derive(Debug)]\n\
         pub struct HeuristicRule {{\n\
             pub language: &'static str,\n\
             pub patterns: &'static [&'static str],\n\
             pub named_pattern: Option<&'static str>,\n\
             pub negative_patterns: &'static [&'static str],\n\
             pub and: &'static [HeuristicRuleCondition],\n\
         }}\n\
         \n\
         #[derive(Debug)]\n\
         pub struct HeuristicEntry {{\n\
             pub extensions: &'static [&'static str],\n\
             pub rules: &'static [HeuristicRule],\n\
         }}\n"
    )
}

/// Write the named pattern lookup table to the generated source.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn write_named_patterns_section(code: &mut String, heuristics: &HeuristicsFile) -> fmt::Result {
    writeln!(
        code,
        "/// Named pattern definitions provided by GitHub Linguist.\n\
         pub const NAMED_HEURISTIC_PATTERNS: &[(&str, &[&str])] = &["
    )?;

    for (name, patterns) in &heuristics.named_patterns {
        writeln!(code, "    (\"{}\", &[", escape_rust_string(name.as_str()))?;
        for pattern in patterns.as_slice() {
            writeln!(
                code,
                "        \"{}\",",
                escape_rust_string(pattern.as_str())
            )?;
        }
        writeln!(code, "    ]),")?;
    }

    writeln!(code, "];\n")
}

/// Write the heuristics grouped by ambiguous extensions.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn write_disambiguations_section(code: &mut String, heuristics: &HeuristicsFile) -> fmt::Result {
    writeln!(
        code,
        "/// Disambiguation heuristics grouped by ambiguous extension.\n\
         pub const LANGUAGE_DETECTION_HEURISTICS: &[HeuristicEntry] = &["
    )?;

    for disamb in &heuristics.disambiguations {
        let mut sorted_exts: Vec<_> = disamb.extensions.iter().map(String::as_str).collect();
        sorted_exts.sort_unstable();

        writeln!(code, "    HeuristicEntry {{")?;
        writeln!(code, "        extensions: &[")?;
        for ext in sorted_exts {
            writeln!(code, "            \"{}\",", escape_rust_string(ext))?;
        }
        writeln!(code, "        ],")?;

        write_rules_section(code, &disamb.rules)?;
        writeln!(code, "    }},")?;
    }

    writeln!(code, "];")
}

/// Write the rule list for a particular ambiguous extension group.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn write_rules_section(code: &mut String, rules: &[Rule]) -> fmt::Result {
    writeln!(code, "        rules: &[")?;
    for rule in rules {
        for language in &rule.language {
            writeln!(code, "            HeuristicRule {{")?;
            writeln!(
                code,
                "                language: \"{}\",",
                escape_rust_string(language)
            )?;

            let patterns: Vec<String> = rule
                .pattern
                .as_ref()
                .map_or_else(Vec::new, |list| list.as_slice().to_vec());
            push_pattern_slice(code, "                patterns", &patterns)?;

            if let Some(name) = rule.named_pattern.as_ref() {
                writeln!(
                    code,
                    "                named_pattern: Some(\"{}\"),",
                    escape_rust_string(name)
                )?;
            } else {
                writeln!(code, "                named_pattern: None,")?;
            }

            let negative_patterns: Vec<String> = rule
                .negative_pattern
                .as_ref()
                .map_or_else(Vec::new, |list| list.as_slice().to_vec());
            push_pattern_slice(
                code,
                "                negative_patterns",
                &negative_patterns,
            )?;

            write_rule_conditions(code, &rule.and)?;
            writeln!(code, "            }},")?;
        }
    }
    writeln!(code, "        ],")
}

/// Write nested rule conditions representing logical AND clauses.
///
/// # Errors
///
/// Returns an error if writing into the string buffer fails.
fn write_rule_conditions(code: &mut String, conditions: &[RuleCondition]) -> fmt::Result {
    if conditions.is_empty() {
        return writeln!(code, "                and: &[],");
    }

    writeln!(code, "                and: &[")?;
    for condition in conditions {
        writeln!(code, "                    HeuristicRuleCondition {{")?;

        let condition_patterns: Vec<String> = condition
            .pattern
            .as_ref()
            .map_or_else(Vec::new, |list| list.as_slice().to_vec());
        push_pattern_slice(
            code,
            "                        patterns",
            &condition_patterns,
        )?;

        if let Some(name) = condition.named_pattern.as_ref() {
            writeln!(
                code,
                "                        named_pattern: Some(\"{}\"),",
                escape_rust_string(name)
            )?;
        } else {
            writeln!(code, "                        named_pattern: None,")?;
        }

        let condition_negative_patterns: Vec<String> = condition
            .negative_pattern
            .as_ref()
            .map_or_else(Vec::new, |list| list.as_slice().to_vec());
        push_pattern_slice(
            code,
            "                        negative_patterns",
            &condition_negative_patterns,
        )?;

        writeln!(code, "                    }},")?;
    }
    writeln!(code, "                ],")
}

/// Generate Rust code for language metadata from languages.yml.
///
/// # Errors
///
/// Returns an error if the generated code cannot be written to the string buffer.
#[allow(
    clippy::too_many_lines,
    reason = "Code generation function necessarily iterates all metadata fields"
)]
#[allow(
    clippy::pattern_type_mismatch,
    reason = "Matching against &Option is clearer than dereferencing"
)]
fn generate_languages_metadata_code(languages: &BTreeMap<String, Language>) -> Result<String> {
    let mut code = String::new();

    writeln!(
        code,
        "// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY\n\
         // Generated from GitHub Linguist languages.yml\n\
         // Run: cargo run --bin sync-linguist --features sync-tool\n\
         \n\
         //! Auto-generated language metadata from GitHub Linguist.\n\
         //! Contains comprehensive language definitions including extensions,\n\
         //! colors, interpreters, and editor modes.\n\
         \n\
         #[derive(Debug, Clone)]\n\
         pub struct LanguageMetadata {{\n\
             pub name: &'static str,\n\
             pub language_type: &'static str,\n\
             pub color: Option<&'static str>,\n\
             pub extensions: &'static [&'static str],\n\
             pub aliases: &'static [&'static str],\n\
             pub interpreters: &'static [&'static str],\n\
             pub filenames: &'static [&'static str],\n\
             pub ace_mode: Option<&'static str>,\n\
             pub tm_scope: Option<&'static str>,\n\
             pub language_id: Option<i64>,\n\
             pub codemirror_mode: Option<&'static str>,\n\
             pub codemirror_mime_type: Option<&'static str>,\n\
             pub group: Option<&'static str>,\n\
             pub wrap: Option<bool>,\n\
             pub fs_name: Option<&'static str>,\n\
         }}\n\
         \n\
         /// All language definitions from GitHub Linguist.\n\
         pub const LANGUAGES: &[LanguageMetadata] = &["
    )?;

    for (name, lang) in languages {
        writeln!(code, "    LanguageMetadata {{")?;
        writeln!(code, "        name: \"{}\",", escape_rust_string(name))?;
        writeln!(
            code,
            "        language_type: \"{}\",",
            escape_rust_string(&lang.language_type)
        )?;

        if let Some(color) = &lang.color {
            writeln!(
                code,
                "        color: Some(\"{}\"),",
                escape_rust_string(color)
            )?;
        } else {
            writeln!(code, "        color: None,")?;
        }

        writeln!(code, "        extensions: &[")?;
        for ext in &lang.extensions {
            writeln!(code, "            \"{}\",", escape_rust_string(ext))?;
        }
        writeln!(code, "        ],")?;

        writeln!(code, "        aliases: &[")?;
        for alias in &lang.aliases {
            writeln!(code, "            \"{}\",", escape_rust_string(alias))?;
        }
        writeln!(code, "        ],")?;

        writeln!(code, "        interpreters: &[")?;
        for interp in &lang.interpreters {
            writeln!(code, "            \"{}\",", escape_rust_string(interp))?;
        }
        writeln!(code, "        ],")?;

        writeln!(code, "        filenames: &[")?;
        for filename in &lang.filenames {
            writeln!(code, "            \"{}\",", escape_rust_string(filename))?;
        }
        writeln!(code, "        ],")?;

        if let Some(ace_mode) = &lang.ace_mode {
            writeln!(
                code,
                "        ace_mode: Some(\"{}\"),",
                escape_rust_string(ace_mode)
            )?;
        } else {
            writeln!(code, "        ace_mode: None,")?;
        }

        if let Some(tm_scope) = &lang.tm_scope {
            writeln!(
                code,
                "        tm_scope: Some(\"{}\"),",
                escape_rust_string(tm_scope)
            )?;
        } else {
            writeln!(code, "        tm_scope: None,")?;
        }

        if let Some(id) = lang.language_id {
            writeln!(code, "        language_id: Some({id}),")?;
        } else {
            writeln!(code, "        language_id: None,")?;
        }

        if let Some(mode) = &lang.codemirror_mode {
            writeln!(
                code,
                "        codemirror_mode: Some(\"{}\"),",
                escape_rust_string(mode)
            )?;
        } else {
            writeln!(code, "        codemirror_mode: None,")?;
        }

        if let Some(mime) = &lang.codemirror_mime_type {
            writeln!(
                code,
                "        codemirror_mime_type: Some(\"{}\"),",
                escape_rust_string(mime)
            )?;
        } else {
            writeln!(code, "        codemirror_mime_type: None,")?;
        }

        if let Some(group) = &lang.group {
            writeln!(
                code,
                "        group: Some(\"{}\"),",
                escape_rust_string(group)
            )?;
        } else {
            writeln!(code, "        group: None,")?;
        }

        if let Some(wrap) = lang.wrap {
            writeln!(code, "        wrap: Some({wrap}),")?;
        } else {
            writeln!(code, "        wrap: None,")?;
        }

        if let Some(fs_name) = &lang.fs_name {
            writeln!(
                code,
                "        fs_name: Some(\"{}\"),",
                escape_rust_string(fs_name)
            )?;
        } else {
            writeln!(code, "        fs_name: None,")?;
        }

        writeln!(code, "    }},")?;
    }

    writeln!(code, "];")?;
    Ok(code)
}

/// Persist generated source to disk, creating parent directories on demand.
///
/// # Errors
///
/// Returns an error when filesystem calls fail.
fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    if path
        .parent()
        .is_some_and(|parent| parent != Path::new(".") && !parent.exists())
    {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create parent directory for {}", path.display())
            })?;
        }
    }

    fs::write(path, contents).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Entry point for the sync utility.
///
/// # Errors
///
/// Propagates HTTP, parsing, and filesystem errors.
///
/// # Panics
///
/// Panics only if `StringList` encounters an unexpected variant (which should be
/// impossible with the current serde definitions).
#[allow(
    clippy::too_many_lines,
    reason = "Main function orchestrates multiple sync phases"
)]
#[tokio::main]
async fn main() -> Result<()> {
    if env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .try_init()
        .is_err()
    {
        // Already initialized elsewhere; nothing to do.
    }

    info!("🚀 Syncing patterns from GitHub Linguist…");

    let vendor_content = fetch_url(VENDOR_YML_URL).await?;
    let generated_content = fetch_url(GENERATED_RB_URL).await?;
    let heuristics_content = fetch_url(HEURISTICS_YML_URL).await?;
    let languages_content = fetch_url(LANGUAGES_YML_URL).await?;

    info!("⚙️  Parsing vendor patterns…");
    let vendor_patterns = parse_vendor_yml(&vendor_content)?;
    info!("   → {} vendored patterns", vendor_patterns.len());

    info!("⚙️  Parsing generated patterns…");
    let generated_patterns = parse_generated_rb(&generated_content)?;
    let (generated, binary) = split_generated_patterns(&generated_patterns);
    info!(
        "   → {} generated patterns ({} binary extensions)",
        generated.len(),
        binary.len()
    );

    info!("⚙️  Parsing heuristics…");
    let heuristics = parse_heuristics_yml(&heuristics_content)?;
    info!(
        "   → {} disambiguation groups, {} named patterns",
        heuristics.disambiguations.len(),
        heuristics.named_patterns.len()
    );

    info!("⚙️  Parsing languages metadata…");
    let languages: BTreeMap<String, Language> =
        serde_yaml::from_str(&languages_content).context("Failed to parse languages.yml")?;
    info!("   → {} languages", languages.len());

    info!("💾 Generating Rust source files…");
    let file_classifier_code =
        generate_file_classifier_code(&vendor_patterns, &generated, &binary)?;
    let heuristics_code = generate_heuristics_code(&heuristics)?;
    let languages_code = generate_languages_metadata_code(&languages)?;

    write_to_file(Path::new(FILE_CLASSIFIER_PATH), &file_classifier_code)?;
    write_to_file(Path::new(HEURISTICS_PATH), &heuristics_code)?;
    write_to_file(Path::new(LANGUAGES_METADATA_PATH), &languages_code)?;
    write_to_file(Path::new(LANGUAGES_YML_OUTPUT_PATH), &languages_content)?;

    info!("✅ Wrote {}", PathBuf::from(FILE_CLASSIFIER_PATH).display());
    info!("✅ Wrote {}", PathBuf::from(HEURISTICS_PATH).display());
    info!(
        "✅ Wrote {}",
        PathBuf::from(LANGUAGES_METADATA_PATH).display()
    );
    info!(
        "✅ Wrote {}",
        PathBuf::from(LANGUAGES_YML_OUTPUT_PATH).display()
    );
    info!("📍 Sync complete!");

    Ok(())
}
