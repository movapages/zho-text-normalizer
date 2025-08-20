//! Kangxi radical normalization

use crate::types::{ChangeType, NormalizedText, TextChange};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Normalizer for Kangxi radicals
pub struct KangxiNormalizer {
    kangxi_map: HashMap<char, char>,
}

impl KangxiNormalizer {
    /// Create a new Kangxi normalizer
    pub fn new() -> Self {
        Self {
            kangxi_map: Self::load_kangxi_mappings(),
        }
    }

    /// Normalize Kangxi radicals in the given text
    pub fn normalize(&self, text: &str) -> NormalizedText {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut changes = Vec::new();

        for (pos, &ch) in chars.iter().enumerate() {
            if let Some(&normalized) = self.kangxi_map.get(&ch) {
                result.push(normalized);
                changes.push(TextChange {
                    position: pos,
                    original_char: ch,
                    normalized_char: normalized,
                    change_type: ChangeType::KangxiRadical,
                    reason: format!("Kangxi radical {} → standard character {}", ch, normalized),
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

    /// Load Kangxi mappings from the separated mapping file
    fn load_kangxi_mappings() -> HashMap<char, char> {
        let mut kangxi_map = HashMap::new();

        // Try to load from the Kangxi mappings file
        let mappings_path = Path::new("data/processed/kangxi_mappings.json");

        if let Ok(contents) = fs::read_to_string(mappings_path) {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (kangxi, standard) in mappings {
                    if let (Some(kangxi_char), Some(standard_char)) =
                        (kangxi.chars().next(), standard.chars().next())
                    {
                        kangxi_map.insert(kangxi_char, standard_char);
                    }
                }
            }
        }

        // Fallback to hardcoded mappings if file doesn't exist
        if kangxi_map.is_empty() {
            // Kangxi Radicals (U+2F00-U+2FDF) to Standard Characters
            // Radical 1-20
            kangxi_map.insert('⼀', '一'); // Radical 1
            kangxi_map.insert('丨', '丨'); // Radical 2
            kangxi_map.insert('丶', '丶'); // Radical 3
            kangxi_map.insert('丿', '丿'); // Radical 4
            kangxi_map.insert('乙', '乙'); // Radical 5
            kangxi_map.insert('亅', '亅'); // Radical 6
            kangxi_map.insert('二', '二'); // Radical 7
            kangxi_map.insert('亠', '亠'); // Radical 8
            kangxi_map.insert('人', '人'); // Radical 9
            kangxi_map.insert('儿', '儿'); // Radical 10
            kangxi_map.insert('入', '入'); // Radical 11
            kangxi_map.insert('八', '八'); // Radical 12
            kangxi_map.insert('冂', '冂'); // Radical 13
            kangxi_map.insert('冖', '冖'); // Radical 14
            kangxi_map.insert('冫', '冫'); // Radical 15
            kangxi_map.insert('几', '几'); // Radical 16
            kangxi_map.insert('凵', '凵'); // Radical 17
            kangxi_map.insert('刀', '刀'); // Radical 18
            kangxi_map.insert('力', '力'); // Radical 19
            kangxi_map.insert('勹', '勹'); // Radical 20
        }

        kangxi_map
    }
}

impl Default for KangxiNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangxi_normalization() {
        let normalizer = KangxiNormalizer::new();
        let result = normalizer.normalize("⽅⾯問題");

        assert_eq!(result.normalized, "方面問題");
        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.changes[0].change_type, ChangeType::KangxiRadical);
        assert_eq!(result.changes[1].change_type, ChangeType::KangxiRadical);
    }

    #[test]
    fn test_no_kangxi_characters() {
        let normalizer = KangxiNormalizer::new();
        let result = normalizer.normalize("普通文字");

        assert_eq!(result.normalized, "普通文字");
        assert!(result.changes.is_empty());
    }

    #[test]
    fn test_mixed_kangxi_and_normal() {
        let normalizer = KangxiNormalizer::new();
        let result = normalizer.normalize("⽅面⽅面");

        assert_eq!(result.normalized, "方面方面");
        assert_eq!(result.changes.len(), 2);
    }
}
