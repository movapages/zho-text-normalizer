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
        // Try multiple possible paths
        let possible_paths = [
            "data/processed/kangxi_mappings.json",
            "../zho-text-normalizer/data/processed/kangxi_mappings.json",
            "zho-text-normalizer/data/processed/kangxi_mappings.json",
        ];

        let mut mappings_path = None;
        for path in &possible_paths {
            if Path::new(path).exists() {
                mappings_path = Some(Path::new(path));
                break;
            }
        }

        let mappings_path =
            mappings_path.unwrap_or(Path::new("data/processed/kangxi_mappings.json"));

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

        // No fallback needed - JSON file is always generated and committed to Git
        if kangxi_map.is_empty() {
            eprintln!("Warning: Failed to load Kangxi mappings from JSON file");
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
