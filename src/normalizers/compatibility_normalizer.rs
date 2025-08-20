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

    /// Load compatibility mappings from the separated mapping file
    fn load_compatibility_mappings() -> HashMap<char, char> {
        let mut compatibility_map = HashMap::new();

        // Try to load from the compatibility mappings file
        let mappings_path = Path::new("data/processed/compatibility_mappings.json");

        if let Ok(contents) = fs::read_to_string(mappings_path) {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (compatibility, standard) in mappings {
                    if let (Some(compatibility_char), Some(standard_char)) =
                        (compatibility.chars().next(), standard.chars().next())
                    {
                        compatibility_map.insert(compatibility_char, standard_char);
                    }
                }
            }
        }

        // Fallback to hardcoded mappings if file doesn't exist
        if compatibility_map.is_empty() {
            // Common compatibility forms
            compatibility_map.insert('㐀', '一');
            compatibility_map.insert('㐁', '丁');
            compatibility_map.insert('㐂', '七');
            compatibility_map.insert('㐃', '万');
            compatibility_map.insert('㐄', '丈');
            compatibility_map.insert('㐅', '三');
            compatibility_map.insert('㐆', '上');
            compatibility_map.insert('㐇', '下');
            compatibility_map.insert('㐈', '不');
            compatibility_map.insert('㐉', '与');
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
        let result = normalizer.normalize("㐀㐁㐂㐃㐄㐅㐆㐇㐈㐉");

        assert_eq!(result.normalized, "一丁七万丈三上下不与");
        assert_eq!(result.changes.len(), 10);
        assert_eq!(result.changes[0].change_type, ChangeType::CompatibilityForm);
    }

    #[test]
    fn test_no_compatibility_characters() {
        let normalizer = CompatibilityNormalizer::new();
        let result = normalizer.normalize("普通文字");

        assert_eq!(result.normalized, "普通文字");
        assert!(result.changes.is_empty());
    }
}
