//! Examples of using the Singularity Language Registry
//!
//! Run with: `cargo run --example usage`

use singularity_language_registry::{
    ast_grep_supported_languages, detect_from_content, detect_language, get_language,
    get_language_by_alias, is_detectable, languages_by_families, rca_supported_languages,
    recommended_linters, same_family, supported_languages, supports_feature, AnalysisFeature,
    LanguageStats,
};
use std::path::Path;

fn main() {
    println!("=== Singularity Language Registry Examples ===\n");

    // 1. Basic language detection
    println!("1. Language Detection:");
    let files = vec!["example.rs", "test.ex", "app.js", "main.go", "lib.gleam"];
    for file in files {
        let path = Path::new(file);
        match detect_language(path) {
            Ok(lang) => {
                println!("  {} -> {} (family: {:?})", file, lang.name, lang.family);
            }
            Err(_) => println!("  {} -> Unknown", file),
        }
    }

    // 2. Language lookup
    println!("\n2. Language Lookup:");
    if let Some(elixir) = get_language("elixir") {
        println!("  Elixir extensions: {:?}", elixir.extensions);
        println!("  RCA supported: {}", elixir.rca_supported);
        println!("  AST-Grep supported: {}", elixir.ast_grep_supported);
    }

    // 3. Alias lookup
    println!("\n3. Alias Lookup:");
    if let Some(js) = get_language_by_alias("js") {
        println!("  'js' alias -> {}", js.name);
    }

    // 4. Language statistics
    println!("\n4. Language Statistics:");
    let stats = LanguageStats::calculate();
    println!("  Total languages: {}", stats.total_languages);
    println!("  RCA supported: {}", stats.rca_supported);
    println!("  AST-Grep supported: {}", stats.ast_grep_supported);
    println!("  Compiled languages: {}", stats.compiled_languages);
    println!("  Language families: {}", stats.families);

    // 5. Language families
    println!("\n5. Language Families:");
    let families = languages_by_families();
    for (family, langs) in families.iter() {
        let lang_names: Vec<&str> = langs.iter().map(|l| l.name.as_str()).collect();
        println!("  {} family: {:?}", family, lang_names);
    }

    // 6. Content detection
    println!("\n6. Content Detection:");
    let python_content = "#!/usr/bin/env python3\nimport sys\nprint('Hello')";
    if let Some(lang) = detect_from_content(python_content) {
        println!("  Shebang detection -> {}", lang.name);
    }

    let dockerfile_content = "FROM rust:latest\nRUN cargo build";
    if let Some(lang) = detect_from_content(dockerfile_content) {
        println!("  Pattern detection -> {}", lang.name);
    }

    // 7. Detection confidence
    println!("\n7. Detection Confidence:");
    let test_path = Path::new("main.rs");
    let detectable = is_detectable(test_path, None);
    println!(
        "  main.rs detectable: {}",
        if detectable { "Yes" } else { "No" }
    );

    // 8. Same family check
    println!("\n8. Same Family Check:");
    println!(
        "  Elixir & Erlang same family? {}",
        same_family("elixir", "erlang")
    );
    println!(
        "  Rust & Python same family? {}",
        same_family("rust", "python")
    );

    // 9. Recommended linters
    println!("\n9. Recommended Linters:");
    for lang in ["rust", "elixir", "javascript"] {
        let linters = recommended_linters(lang);
        println!("  {} -> {:?}", lang, linters);
    }

    // 10. Feature support
    println!("\n10. Feature Support:");
    let features = [
        ("rust", AnalysisFeature::RCA),
        ("elixir", AnalysisFeature::ASTGrep),
        ("python", AnalysisFeature::Complexity),
    ];
    for (lang, feature) in features {
        let supported = supports_feature(lang, feature);
        println!("  {} supports {:?}: {}", lang, feature, supported);
    }

    // 11. List all supported languages
    println!("\n11. All Supported Languages:");
    let all = supported_languages();
    println!("  Total: {} languages", all.len());
    let names: Vec<&str> = all.iter().take(5).map(|l| l.name.as_str()).collect();
    println!("  First 5: {:?} ...", names);

    // 12. RCA supported languages
    println!("\n12. RCA Supported Languages:");
    let rca = rca_supported_languages();
    let rca_names: Vec<&str> = rca.iter().map(|l| l.name.as_str()).collect();
    println!("  RCA languages: {:?}", rca_names);

    // 13. AST-Grep supported languages
    println!("\n13. AST-Grep Supported Languages:");
    let ast = ast_grep_supported_languages();
    println!("  AST-Grep supports {} languages", ast.len());

    println!("\n=== Examples Complete ===");
}
