# ZHO Text Normalizer

[![Crates.io](https://img.shields.io/crates/v/zho-text-normalizer)](https://crates.io/crates/zho-text-normalizer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/movapages/zho-text-normalizer/actions/workflows/rust.yml/badge.svg)](https://github.com/movapages/zho-text-normalizer/actions/workflows/rust.yml)

A comprehensive Chinese text normalizer with Unicode support, built in Rust. Combines OpenCC's battle-tested script conversion with Unihan-based character normalization.

## Features

- **Script Conversion**: Traditional ↔ Simplified Chinese conversion using OpenCC
- **Kangxi Radical Normalization**: Converts Unicode Kangxi radicals to standard characters
- **Character Variant Normalization**: Normalizes character variants using Unihan data
- **Compatibility Form Normalization**: Converts compatibility characters to standard forms
- **Unicode Normalization**: NFC normalization for consistent text representation
- **Script Detection**: Auto-detects Traditional/Simplified Chinese text
- **Detailed Change Tracking**: Records all transformations with explanations

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zho-text-normalizer = { git = "https://github.com/movapages/zho-text-normalizer" }
```

Or via command line:
```bash
cargo add --git https://github.com/movapages/zho-text-normalizer
```

For development:
```bash
# Clone the repository
git clone https://github.com/movapages/zho-text-normalizer
cd zho-text-normalizer

# Build the library
cargo build --release

# Run tests
cargo test --workspace

### Basic Usage

```rust
use zho_text_normalizer::{normalize, normalize_to_script, Script};

// Simple normalization (auto-detects script)
let result = normalize("⽅⾯問題");
println!("Normalized: {}", result.normalized);

// Convert to Simplified Chinese
let result = normalize_to_script("這個藥", Script::SimplifiedChinese);
println!("Simplified: {}", result.normalized);
```

### Command Line Interface

```bash
# Install CLI tool
cargo install zho-text-normalizer

# Basic normalization (detects script automatically)
zho-normalize "⽅⾯問題"

# Convert Traditional to Simplified
zho-normalize --target simplified "這個藥一天喫四回，或是五回都可以。"

# Show detailed changes
zho-normalize --format verbose "⼀、本書以說明規律為主"

# Validation mode (no conversion, just analysis)
zho-normalize --validate "中國現代語法"
```

## Advanced Usage

### Custom Normalization Pipeline

```rust
use zho_text_normalizer::{TextNormalizer, Script};

// Create a normalizer
let normalizer = TextNormalizer::new();

// Normalize with detailed change tracking
let result = normalizer.normalize("這個藥", Some(Script::SimplifiedChinese));

// Inspect changes
for change in result.changes {
    println!("{} → {} ({})", 
        change.original_char,
        change.normalized_char,
        change.reason
    );
}
```

## Normalization Pipeline

The normalizer processes text through the following steps:

1. **Script Detection**: Identifies Traditional/Simplified Chinese
2. **Unicode Normalization**: Applies NFC normalization
3. **Kangxi Radical Normalization**: `⽅` → `方`
4. **Character Variant Normalization**: `敎` → `教`
5. **Compatibility Form Normalization**: `㐀` → `一`
6. **Script Conversion**: Uses OpenCC for Traditional ↔ Simplified conversion

## Examples

### Script Conversion
```rust
// Traditional → Simplified
"這個藥一天喫四回" → "这个药一天吃四回"

// Simplified → Traditional
"这个药一天吃四回" → "這個藥一天喫四回"
```

### Kangxi Radical Normalization
```rust
"⼀、本書" → "一、本書"  // Kangxi radical → standard character
```

### Character Variant Normalization
```rust
"敎育" → "教育"  // Variant form → standard form
```

## Data Sources

- **OpenCC**: Primary source for Traditional ↔ Simplified conversion
- **Unihan Database**: Character variants and compatibility mappings
- **Unicode Standard**: Kangxi radical mappings and normalization forms

## System Requirements

- Rust 1.70+ (2021 edition)
- OpenCC library (automatically installed via build script)
- Unihan database (automatically downloaded and processed at build time)

## Data Files

The normalizer uses several data files that are generated at build time:

- `data/processed/script_mappings.json`: Traditional ↔ Simplified mappings
- `data/processed/variant_mappings.json`: Character variant mappings
- `data/processed/kangxi_mappings.json`: Kangxi radical mappings
- `data/processed/compatibility_mappings.json`: Compatibility form mappings

These files are generated from the Unihan database automatically during the build process. If you need to regenerate them manually:

```bash
cargo run --bin process-unihan
```

Note: The data files are ignored by git. When using this library as a dependency, the data files will be generated automatically during the build process.

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --workspace`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

Please make sure to update tests as appropriate and follow the existing coding style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenCC](https://github.com/BYVoid/OpenCC) for providing the core script conversion functionality
- [Unicode Consortium](https://www.unicode.org/) for the Unihan Database