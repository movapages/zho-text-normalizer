# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-08-24

### Added

#### **🎯 Comprehensive Character Mappings**
- **13,317 Script Mappings**: Traditional ↔ Simplified conversion via OpenCC integration
- **4,122 Variant Mappings**: Extracted from Unicode Unihan database
  - 3,378 Semantic variants (`kSemanticVariant`)
  - 68 Spoofing variants (`kSpoofingVariant`) 
  - 118 Z-variants (`kZVariant`)
  - 558 Specialized variants (`kSpecializedSemanticVariant`)
- **820 Compatibility Mappings**: Real `kCompatibilityVariant` from Unihan IRGSources
- **214 Kangxi Radicals**: Complete Unicode Kangxi radical → standard character conversion

#### **🧠 Smart Processing Features**
- **Confidence-Based Filtering**: Multi-source validation prevents over-normalization
- **Script Auto-Detection**: Intelligent Traditional/Simplified Chinese detection
- **Unicode Normalization**: NFC normalization for consistent text representation
- **Detailed Change Tracking**: Complete audit trail with position, type, and reason

#### **⚡ Performance & Architecture**
- **Git-Friendly Distribution**: All mapping data included in repository (47.5MB)
- **Zero External Dependencies**: Self-contained with embedded Unicode data
- **Fast JSON Loading**: Optimized parsing for quick startup
- **Modular Design**: Individual normalizers can be used independently

#### **🔧 Developer Tools**
- **CLI Tool**: `zho-normalize` binary for command-line usage
- **Data Processor**: `process-unihan` for regenerating mappings from source
- **Comprehensive Examples**: Basic and advanced usage patterns
- **Full API Documentation**: Rust doc comments for all public interfaces

#### **📊 Data Sources**
- **Unicode Unihan Database**: Official Unicode character data (Version 16.0.0)
- **OpenCC Integration**: Battle-tested Traditional/Simplified conversion
- **Authentic Mappings**: Direct extraction from authoritative sources

### Technical Details

#### **Normalization Pipeline**
1. **Unicode Normalization** (NFC)
2. **Kangxi Radical Conversion** (U+2F00-U+2FD5 → standard characters)
3. **Character Variant Normalization** (confidence-based selection)
4. **Compatibility Form Conversion** (CJK compatibility ideographs)
5. **Script Conversion** (Traditional ↔ Simplified, optional)

#### **Data Processing**
- Extracted from 8 Unihan database files (40MB source data)
- Generated 8 optimized JSON mapping files (7.5MB processed data)
- Implemented confidence scoring based on source dictionary count
- Added bidirectional mapping validation

#### **Quality Assurance**
- Comprehensive test coverage for all normalizers
- Validation against Unicode decomposition standards
- Performance benchmarking and optimization
- Git-friendly text format for maintainability

### Dependencies

- `clap` 4.4+ - Command-line interface
- `serde` 1.0+ - JSON serialization
- `serde_json` 1.0+ - JSON parsing
- `opencc` 0.1+ - Script conversion
- `unicode-normalization` 0.1+ - Unicode NFC normalization

### Supported Platforms

- **Rust**: 1.70+ (2021 edition)
- **Platforms**: Windows, macOS, Linux
- **Architectures**: x86_64, ARM64
