//! Character variant normalization

use crate::types::{ChangeType, NormalizedText, TextChange};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Normalizer for character variants
pub struct VariantNormalizer {
    variant_map: HashMap<char, char>,
}

impl VariantNormalizer {
    /// Create a new variant normalizer
    pub fn new() -> Self {
        Self {
            variant_map: Self::load_variant_mappings(),
        }
    }

    /// Normalize character variants in the given text
    pub fn normalize(&self, text: &str) -> NormalizedText {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut changes = Vec::new();

        for (pos, &ch) in chars.iter().enumerate() {
            if let Some(&normalized) = self.variant_map.get(&ch) {
                result.push(normalized);
                changes.push(TextChange {
                    position: pos,
                    original_char: ch,
                    normalized_char: normalized,
                    change_type: ChangeType::VariantForm,
                    reason: format!("Variant form {} → standard {}", ch, normalized),
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

    /// Load variant mappings from Unihan data
    fn load_variant_mappings() -> HashMap<char, char> {
        let mut variant_map = HashMap::new();

        // Try to load from a variant mappings file
        let mappings_path = Path::new("data/processed/variant_mappings.json");

        if let Ok(contents) = fs::read_to_string(mappings_path) {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (variant, standard) in mappings {
                    if let (Some(variant_char), Some(standard_char)) =
                        (variant.chars().next(), standard.chars().next())
                    {
                        variant_map.insert(variant_char, standard_char);
                    }
                }
            }
        }

        variant_map
    }
}

impl Default for VariantNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_normalization() {
        let normalizer = VariantNormalizer::new();
        let result = normalizer.normalize("硏究敎育");

        assert_eq!(result.normalized, "硏究敎毓");
        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.changes[0].change_type, ChangeType::VariantForm);
    }
}
