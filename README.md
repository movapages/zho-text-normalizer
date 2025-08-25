# ZHO Text Normalizer

[![Crates.io](https://img.shields.io/crates/v/zho-text-normalizer)](https://crates.io/crates/zho-text-normalizer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/movapages/zho-text-normalizer/actions/workflows/rust.yml/badge.svg)](https://github.com/movapages/zho-text-normalizer/actions/workflows/rust.yml)

A comprehensive Chinese text normalizer with Unicode support, built in Rust. Features **18,473+ character mappings** from authentic Unicode/Unihan data, combining OpenCC's script conversion with advanced character variant normalization.

## Features

### **🎯 Comprehensive Normalization**
- **13,317 Script Mappings**: Traditional ↔ Simplified conversion via OpenCC integration
- **4,122 Variant Mappings**: Semantic, spoofing, Z-variants, and specialized character forms
- **820 Compatibility Mappings**: Real `kCompatibilityVariant` from Unicode Unihan database
- **214 Kangxi Radicals**: Complete Unicode Kangxi radical → standard character conversion

### **🧠 Smart Processing**
- **kIICore Algorithm**: Uses Unicode's International Ideograph Core data to determine standard character forms
- **Confidence-Based Filtering**: Prevents over-normalization with multi-source validation
- **Script Auto-Detection**: Intelligent Traditional/Simplified Chinese detection
- **Unicode Normalization**: NFC normalization for consistent text representation
- **Detailed Change Tracking**: Complete audit trail of all transformations

### **⚡ Performance & Portability**
- **Git-Friendly**: All mapping data included (no external downloads required)
- **Zero Dependencies**: Self-contained with embedded Unicode data
- **Fast Loading**: Optimized JSON parsing for quick startup

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

The normalizer uses a clean, organized data structure with separate files for different purposes:

### Script Conversion Files
- `data/processed/script_conversion/traditional_to_simplified.json`: Traditional → Simplified mappings (5,611 entries)
- `data/processed/script_conversion/simplified_to_traditional.json`: Simplified → Traditional mappings (1,363 entries)
- `data/processed/script_conversion/script_conversion_stats.json`: Conversion statistics

### Normalization Files  
- `data/processed/normalization/semantic_variants.json`: Semantic variant mappings (1,519 entries)
- `data/processed/normalization/compatibility_variants.json`: Compatibility form mappings (1,002 entries)
- `data/processed/normalization/kangxi_radicals.json`: Kangxi radical mappings (214 entries)
- `data/processed/normalization/normalization_stats.json`: Normalization statistics

These files are pre-generated from the official Unicode Unihan database using the kIICore algorithm and **included in the Git repository** for portability. The library works out-of-the-box without requiring external downloads or build scripts.

If you need to regenerate the mappings (e.g., after updating Unihan data):

```bash
cargo run --bin process-unihan --force
```

**Note**: All mapping files are committed to Git, ensuring the library is fully portable and works immediately after cloning.

## Examples

The `examples/` directory contains comprehensive usage examples:

- **`basic_usage.rs`**: Core functionality including script conversion, variant normalization, and change tracking
- **`advanced_usage.rs`**: Individual normalizer components, statistics, batch processing, and validation mode

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example advanced_usage
```

## Algorithm Details

### kIICore-Based Standard Form Detection

The normalizer uses a sophisticated algorithm to determine which character variant is the "standard" form, based on research into Unicode's Unihan database:

#### **1. Primary Rule: Unicode Block Priority**
```
Compatibility Ideographs (U+F900-U+FAFF) → Main CJK (U+4E00-U+9FFF)
```
Characters in compatibility blocks are always normalized to their main CJK equivalents.

#### **2. Secondary Rule: kIICore Region Count**
For characters within the main CJK block, the algorithm uses `kIICore` (International Ideograph Core) data:

```rust
// Example: 回 vs 囘
回 (U+56DE): kIICore = "AGTJHKMP" (8 regions) ← Standard
囘 (U+56D8): No kIICore (0 regions)        ← Variant

Result: 囘 → 回 (variant → standard)
```

**kIICore regions represent international consensus:**
- **A**=China, **G**=Singapore, **T**=Taiwan, **J**=Japan, **H**=Hong Kong, **K**=South Korea, **M**=Macao, **P**=North Korea

#### **3. Validation with Real Text**
The algorithm was validated against actual Chinese literature and news text, ensuring that:
- ✅ Standard forms (一, 回, 三) are preserved
- ✅ Archaic variants (弌, 囘, 叁) normalize to standard forms
- ✅ No over-normalization occurs

#### **4. Separation of Concerns**
- **Script Conversion**: Traditional ↔ Simplified (東 ↔ 东)
- **Variant Normalization**: Archaic → Standard within same script (囘 → 回)
- **Semantic Preservation**: Context-dependent variants (他/她/牠) handled appropriately

This research-based approach ensures linguistically sound normalization that respects both historical usage and modern standards.

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