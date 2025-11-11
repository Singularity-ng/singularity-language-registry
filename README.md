# Singularity Language Registry

[![Documentation](https://img.shields.io/badge/docs-rustdoc-blue.svg)](https://singularity-ng.github.io/singularity-language-registry/)
[![CI](https://github.com/singularity-ng/singularity-language-registry/workflows/CI/badge.svg)](https://github.com/singularity-ng/singularity-language-registry/actions)

Centralized language registry for all Singularity analysis engines.

**[📚 View Documentation](https://singularity-ng.github.io/singularity-language-registry/)** | **[📖 Setup Guide](./DOCS_SETUP.md)**

## Purpose

This crate provides a single source of truth for language definitions across:
- `singularity-parsing-engine` - AST extraction and parsing
- `singularity-analysis-engine` - Code metrics and complexity analysis
- `singularity-code-engine` - Code quality orchestration
- `singularity-linting-engine` - Quality gates and linting

## Design

**Pure language infrastructure** - no dependencies on other Singularity crates.

All engines depend on this registry, not the other way around:

```
language_registry (independent)
    ↑ ↑ ↑ ↑
    │ │ │ └─ parsing_engine
    │ │ └─── analysis_engine
    │ └───── code_engine
    └─────── linting_engine
```

## Features

- **18+ languages** with complete definitions
- **Fast lookups** - extension, alias, and MIME type mapping
- **Family-based grouping** - BEAM, Systems, Web, JVM, C-like, Scripting
- **Capability tracking** - RCA support, AST-Grep support, compilation type
- **Pattern signatures** - language syntax (NOT libraries) for cross-language detection
- **Serializable** - all language info can be exported as JSON

## Installation

### Using Cargo

```bash
cargo add singularity-language-registry
```

### Using Nix

```bash
# Add to your flake.nix inputs
inputs.singularity-language-registry.url = "github:Singularity-ng/singularity-language-registry";

# Or use directly with Nix
nix build github:Singularity-ng/singularity-language-registry

# Enter development shell
nix develop github:Singularity-ng/singularity-language-registry
```

### With FlakeHub (Fastest - Binary Cache)

```bash
# FlakeHub provides pre-built binaries for faster installation
nix build "https://flakehub.com/f/Singularity-ng/singularity-language-registry/*.tar.gz"

# Or add to your flake.nix
inputs.singularity-language-registry.url = "https://flakehub.com/f/Singularity-ng/singularity-language-registry/0.1.*.tar.gz";
```

## Usage

### Basic Language Detection

```rust
use singularity_language_registry::{detect_language, get_language};
use std::path::Path;

// Detect from file path
let path = Path::new("example.rs");
if let Ok(lang) = detect_language(path) {
    println!("Language: {}", lang.name);
    println!("Family: {:?}", lang.family);
}

// Get language by ID
if let Some(lang) = get_language("elixir") {
    println!("Extensions: {:?}", lang.extensions);
    println!("RCA supported: {}", lang.rca_supported);
}
```

### Content-Based Detection

```rust
use singularity_language_registry::{detect_from_content, detect_from_shebang};

let python_code = "#!/usr/bin/env python3\nimport sys";
if let Some(lang) = detect_from_content(python_code, None) {
    println!("Detected: {}", lang.name);
}
```

### Language Statistics & Utilities

```rust
use singularity_language_registry::{
    LanguageStats,
    languages_by_families,
    same_family,
    recommended_linters,
    supports_feature,
    AnalysisFeature,
};

// Get statistics
let stats = LanguageStats::calculate();
println!("Total languages: {}", stats.total_languages);

// Check family relationships
if same_family("elixir", "erlang") {
    println!("Both are BEAM languages!");
}

// Get recommended tools
let linters = recommended_linters("rust");
println!("Rust linters: {:?}", linters);

// Check feature support
if supports_feature("rust", AnalysisFeature::RCA) {
    println!("Rust supports RCA analysis!");
}
```

## API Documentation

### Core Types

- `LanguageInfo` - Complete language metadata (name, extensions, family, etc.)
- `LanguageRegistry` - Main registry struct (usually accessed via LANGUAGE_REGISTRY)
- `PatternSignatures` - Language syntax patterns for detection
- `LanguageStats` - Statistics about language support
- `AnalysisFeature` - Enum of analysis capabilities

### Detection Functions

- `detect_language(path)` - Detect from file path
- `detect_from_content(content, fallback_ext)` - Detect from file content
- `detect_from_shebang(content)` - Detect from shebang line
- `detect_from_patterns(content)` - Detect from content patterns
- `detect_special_files(filename)` - Detect special files (Makefile, Dockerfile, etc.)
- `detection_confidence(path, content)` - Get detection confidence (0.0-1.0)

### Registry Functions

- `get_language(id)` - Get language by ID
- `get_language_by_alias(alias)` - Get language by alias (e.g., "js")
- `get_language_by_mime_type(mime)` - Get language by MIME type
- `supported_languages()` - Get all supported languages
- `rca_supported_languages()` - Get languages with RCA support
- `ast_grep_supported_languages()` - Get languages with AST-Grep support

### Utility Functions

- `languages_by_families()` - Group languages by family
- `same_family(lang1, lang2)` - Check if two languages are in same family
- `recommended_linters(language)` - Get recommended linters for a language
- `file_patterns(language)` - Get common file patterns for a language
- `supports_feature(language, feature)` - Check if language supports a feature

## Supported Languages

- **BEAM**: Elixir, Erlang, Gleam
- **Systems**: Rust, C, C++, Go
- **Web**: JavaScript, TypeScript, Python
- **JVM**: Java, Kotlin
- **Other**: C#, Bash, Lua, SQL, TOML, YAML, JSON, Markdown, Dockerfile

## Zero Dependencies

This crate has no dependencies on other Singularity crates, making it safe for all engines to import without circular dependencies.

## Development

### Documentation

Generate and view documentation locally:

```bash
# Using Nix (recommended)
nix run .#docs

# Using cargo
cargo doc --all-features --open

# Build documentation package with Nix
nix build .#docs
```

The documentation is automatically published to GitHub Pages when changes are pushed to `main`. See [DOCS_SETUP.md](./DOCS_SETUP.md) for complete documentation setup and deployment instructions.

### Testing

```bash
# Run all tests
cargo test --all-features

# Run with Nix
nix flake check
```

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines and contribution process.

## License

**Proprietary Software - All Rights Reserved**

Copyright (c) 2025 Singularity Team. All rights reserved.

This software is proprietary and confidential. Unauthorized copying, distribution, modification, or use of this software, via any medium, is strictly prohibited without explicit written permission from Singularity Team.

See [LICENSE](./LICENSE) for complete terms and conditions.

For licensing inquiries, please contact the Singularity Team.
