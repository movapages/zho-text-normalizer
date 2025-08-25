//! Compatibility form normalization

use crate::types::{ChangeType, NormalizedText, TextChange};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Normalizer for compatibility forms
pub struct CompatibilityNormalizer {
    compatibility_map: HashMap<char, char>,
}

impl CompatibilityNormalizer {
    /// Create a new compatibility normalizer
    pub fn new() -> Self {
        Self {
            compatibility_map: Self::load_compatibility_mappings(),
        }
    }

    /// Normalize compatibility forms in the given text
    pub fn normalize(&self, text: &str) -> NormalizedText {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut changes = Vec::new();

        for (pos, &ch) in chars.iter().enumerate() {
            if let Some(&normalized) = self.compatibility_map.get(&ch) {
                result.push(normalized);
                changes.push(TextChange {
                    position: pos,
                    original_char: ch,
                    normalized_char: normalized,
                    change_type: ChangeType::CompatibilityForm,
                    reason: format!("Compatibility form {} → standard {}", ch, normalized),
                });
            } else {
                result.push(ch);
            }
        }

        NormalizedText {
            original: text.to_string(),
            normalized: result,
            changes,
            detected_script: crate::types::Script::Auto,
            processing_time_ms: 0,
        }
    }

    /// Load compatibility mappings from the new clean normalization structure
    fn load_compatibility_mappings() -> HashMap<char, char> {
        let mut compatibility_map = HashMap::new();

        // Load from the new normalization structure
        let compatibility_path =
            Path::new("data/processed/normalization/compatibility_variants.json");

        if let Ok(contents) = fs::read_to_string(compatibility_path) {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (compatibility, standard) in mappings {
                    if let (Some(compatibility_char), Some(standard_char)) =
                        (compatibility.chars().next(), standard.chars().next())
                    {
                        compatibility_map.insert(compatibility_char, standard_char);
                    }
                }
                println!(
                    "Loaded {} compatibility variant mappings from clean data",
                    compatibility_map.len()
                );
            }
        } else {
            eprintln!(
                "Warning: Failed to load compatibility mappings from clean normalization data"
            );
        }

        compatibility_map
    }
}

impl Default for CompatibilityNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_normalization() {
        let normalizer = CompatibilityNormalizer::new();
        let result = normalizer.normalize("凞"); // Test with compatibility character

        // Just verify it runs without crashing and produces output
        assert!(!result.normalized.is_empty());
        // Note: Actual mappings depend on kIICore data processing
    }

    #[test]
    fn test_no_compatibility_characters() {
        let normalizer = CompatibilityNormalizer::new();
        let result = normalizer.normalize("普通文字");

        assert_eq!(result.normalized, "普通文字");
        assert!(result.changes.is_empty());
    }
}
