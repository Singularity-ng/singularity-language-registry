#!/usr/bin/env python3
"""
Synchronize File Classification Patterns from GitHub Linguist

This script downloads Linguist's vendor.yml and generated.rb files,
extracts patterns, and generates Rust code for the FileClassifier module.

Usage:
    python3 scripts/sync_linguist_patterns.py > src/file_classifier_generated.rs

Sources:
    - vendor.yml: Vendored code path patterns
    - generated.rb: Auto-generated file detection rules
"""

import re
import sys
import urllib.request
from typing import List, Set
import yaml

# GitHub Linguist URLs
VENDOR_YML_URL = "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/vendor.yml"
GENERATED_RB_URL = "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/generated.rb"
HEURISTICS_YML_URL = "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/heuristics.yml"


def fetch_url(url: str) -> str:
    """Fetch content from URL"""
    print(f"Fetching {url}...", file=sys.stderr)
    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            return response.read().decode("utf-8")
    except Exception as e:
        print(f"Error fetching {url}: {e}", file=sys.stderr)
        raise


def parse_vendor_yml(content: str) -> Set[str]:
    """
    Parse vendor.yml and extract vendored path patterns.

    Format:
    ```yaml
    - /path/to/vendor/
    - node_modules/
    - "regex_pattern"
    ```
    """
    patterns: Set[str] = set()

    try:
        data = yaml.safe_load(content)
        if isinstance(data, list):
            for item in data:
                if isinstance(item, str):
                    # Simple path patterns
                    path = item.strip()
                    if path and not path.startswith("#"):
                        patterns.add(path)
    except yaml.YAMLError as e:
        print(f"Error parsing YAML: {e}", file=sys.stderr)
        return patterns

    return patterns


def parse_generated_rb(content: str) -> Set[str]:
    """
    Parse generated.rb and extract generated file patterns.

    Looks for:
    - File extensions: ".pb.rs", ".generated.ts"
    - Directory paths: "__generated__/", "dist/"
    - Content markers for detection
    """
    patterns: Set[str] = set()

    # Pattern to match quoted strings in Ruby
    # Matches: ".pb.rs", '.generated.ts', "pattern"
    string_pattern = re.compile(r'''['"](.*?)['"]''')

    for line in content.split('\n'):
        line = line.strip()

        # Skip comments and empty lines
        if not line or line.startswith('#'):
            continue

        # Extract quoted strings
        matches = string_pattern.findall(line)
        for match in matches:
            if match and len(match) < 50:  # Reasonable pattern length
                patterns.add(match)

    return patterns


def parse_heuristics_yml(content: str) -> dict:
    """
    Parse heuristics.yml for language detection rules.

    This is for Phase 3 (future implementation).
    """
    try:
        data = yaml.safe_load(content)
        return data if data else {}
    except yaml.YAMLError:
        return {}


def categorize_patterns(patterns: Set[str]) -> dict:
    """
    Categorize patterns into:
    - Vendored: node_modules/, vendor/, .yarn/, etc.
    - Generated: .pb.rs, .generated.ts, etc.
    - Binary: .png, .jpg, .exe, etc.
    """
    categories = {
        'vendored': set(),
        'generated': set(),
        'binary': set(),
    }

    binary_extensions = {'.png', '.jpg', '.jpeg', '.gif', '.bmp', '.zip', '.tar', '.exe', '.dll', '.pdf'}

    for pattern in patterns:
        if any(pattern.startswith(v) for v in ['node_modules', 'vendor', '.yarn', '.idea', 'dist', 'build']):
            categories['vendored'].add(pattern)
        elif pattern.startswith('.'):
            # It's an extension
            if any(pattern == ext for ext in binary_extensions):
                categories['binary'].add(pattern)
            elif 'generated' in pattern.lower() or 'pb' in pattern or 'proto' in pattern:
                categories['generated'].add(pattern)
        else:
            categories['vendored'].add(pattern)

    return categories


def generate_rust_code(patterns_dict: dict) -> str:
    """Generate Rust code for patterns"""
    code = '''// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// Generated from GitHub Linguist patterns
// Run: python3 scripts/sync_linguist_patterns.py
// Source: https://github.com/github-linguist/linguist

//! Auto-generated file classification patterns from GitHub Linguist
//!
//! These patterns are synchronized weekly via Renovate.
//! When Linguist updates, run: python3 scripts/sync_linguist_patterns.py

/// Vendored code path patterns (from Linguist vendor.yml)
pub const VENDORED_PATTERNS_FROM_LINGUIST: &[&str] = &[
'''

    for pattern in sorted(patterns_dict['vendored']):
        code += f'    "{pattern}",\n'

    code += '''];

/// Generated file patterns (from Linguist generated.rb)
pub const GENERATED_PATTERNS_FROM_LINGUIST: &[&str] = &[
'''

    for pattern in sorted(patterns_dict['generated']):
        escaped = pattern.replace('"', '\\"').replace('\\', '\\\\')
        code += f'    "{escaped}",\n'

    code += '''];

/// Binary file extensions
pub const BINARY_PATTERNS_FROM_LINGUIST: &[&str] = &[
'''

    for pattern in sorted(patterns_dict['binary']):
        code += f'    "{pattern}",\n'

    code += '''];
'''

    return code


def main():
    """Main entry point"""
    try:
        # Fetch files from Linguist
        print("Synchronizing patterns from GitHub Linguist...", file=sys.stderr)

        vendor_content = fetch_url(VENDOR_YML_URL)
        generated_content = fetch_url(GENERATED_RB_URL)

        # Parse patterns
        vendor_patterns = parse_vendor_yml(vendor_content)
        generated_patterns = parse_generated_rb(generated_content)

        print(f"Found {len(vendor_patterns)} vendor patterns", file=sys.stderr)
        print(f"Found {len(generated_patterns)} generated patterns", file=sys.stderr)

        # Combine and categorize
        all_patterns = vendor_patterns | generated_patterns
        categorized = categorize_patterns(all_patterns)

        # Generate Rust code
        rust_code = generate_rust_code(categorized)

        # Output
        print(rust_code, file=sys.stdout)
        print("// Pattern sync complete!", file=sys.stderr)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
