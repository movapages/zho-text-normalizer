# ZHO Text Normalizer

A comprehensive Chinese text normalizer with Unicode support, built in Rust. Combines OpenCC's battle-tested script conversion with Unihan-based character normalization.

## Features

- **Script Conversion**: Traditional ↔ Simplified Chinese conversion using OpenCC
- **Kangxi Radical Normalization**: Converts Unicode Kangxi radicals to standard characters
- **Character Variant Normalization**: Normalizes character variants using Unihan data
- **Compatibility Form Normalization**: Converts compatibility characters to standard forms
- **Unicode Normalization**: NFC normalization for consistent text representation
- **Script Detection**: Auto-detects Traditional/Simplified Chinese text
- **Detailed Change Tracking**: Records all transformations with explanations

## Installation

### From crates.io
```bash
cargo add zho-text-normalizer
```

### From Source
```bash
git clone https://github.com/your-username/zho-text-normalizer
cd zho-text-normalizer
cargo build --release
```

## Usage

### Command Line

```bash
# Basic normalization (detects script automatically)
zho-normalize "⽅⾯問題"

# Convert Traditional to Simplified
zho-normalize --target simplified "這個藥一天喫四回，或是五回都可以。"

# Show detailed changes
zho-normalize --format verbose "⼀、本書以說明規律為主"

# Validation mode (no conversion, just analysis)
zho-normalize --validate "中國現代語法"
```

### As a Library

```rust
use zho_text_normalizer::{TextNormalizer, Script};

// Create a normalizer
let normalizer = TextNormalizer::new();

// Basic normalization (auto-detects script)
let result = normalizer.normalize("⽅⾯問題", None);
println!("Normalized: {}", result.normalized);

// Convert Traditional to Simplified
let result = normalizer.normalize("這個藥", Some(Script::SimplifiedChinese));
println!("Simplified: {}", result.normalized);

// Get detailed changes
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

## Dependencies

- Rust 1.70+ (2021 edition)
- OpenCC library
- Unihan database (processed at build time)

### Data Files

The normalizer uses several data files that are generated at build time:

- `data/processed/script_mappings.json`: Traditional ↔ Simplified mappings
- `data/processed/variant_mappings.json`: Character variant mappings
- `data/processed/kangxi_mappings.json`: Kangxi radical mappings
- `data/processed/compatibility_mappings.json`: Compatibility form mappings

These files are generated from the Unihan database by running:
```bash
cargo run --bin process-unihan
```

Note: The data files are ignored by git. When using this library as a dependency, the data files will be generated automatically during the build process.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.