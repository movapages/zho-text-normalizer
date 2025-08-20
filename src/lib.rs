//! ZHO Text Normalizer - Comprehensive Chinese text normalization
//!
//! This library provides comprehensive text normalization for Chinese text, including:
//! - Traditional ↔ Simplified script conversion (via OpenCC)
//! - Kangxi radical normalization
//! - Character variant normalization (via Unihan)
//! - Compatibility form normalization
//! - Unicode NFC normalization

pub mod normalizers;
pub mod types;
pub mod utils;

pub use normalizers::text_normalizer::TextNormalizer;
pub use types::{NormalizedText, Script};

/// Normalize text with default configuration
pub fn normalize(text: &str) -> NormalizedText {
    let normalizer = TextNormalizer::new();
    normalizer.normalize(text, None)
}

/// Normalize text with target script
pub fn normalize_to_script(text: &str, target_script: Script) -> NormalizedText {
    let normalizer = TextNormalizer::new();
    normalizer.normalize(text, Some(target_script))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_normalization() {
        let result = normalize("⽅⾯問題");
        assert_eq!(result.normalized, "方面問題");
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_variant_normalization() {
        let result = normalize("硏究敎育");
        assert_eq!(result.normalized, "硏究敎毓");
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_script_conversion() {
        let result = normalize_to_script("這個藥", Script::SimplifiedChinese);
        assert_ne!(result.normalized, "這個藥");
        assert!(!result.changes.is_empty());
    }
}
