//! Unicode normalization

use crate::types::{ChangeType, NormalizedText, TextChange, UnicodeNormalization};
use unicode_normalization::UnicodeNormalization as UnicodeNorm;

/// Normalizer for Unicode normalization forms
pub struct UnicodeNormalizer;

impl UnicodeNormalizer {
    /// Create a new Unicode normalizer
    pub fn new() -> Self {
        Self
    }

    /// Normalize text using the specified Unicode normalization form
    pub fn normalize(&self, text: &str, form: UnicodeNormalization) -> NormalizedText {
        let normalized = match form {
            UnicodeNormalization::NFC => text.nfc().collect::<String>(),
            UnicodeNormalization::NFD => text.nfd().collect::<String>(),
            UnicodeNormalization::NFKC => text.nfkc().collect::<String>(),
            UnicodeNormalization::NFKD => text.nfkd().collect::<String>(),
            UnicodeNormalization::None => text.to_string(),
        };

        let changes = if normalized != text {
            // Calculate changes by comparing character by character
            let mut changes = Vec::new();
            let original_chars: Vec<char> = text.chars().collect();
            let normalized_chars: Vec<char> = normalized.chars().collect();

            let max_len = original_chars.len().max(normalized_chars.len());

            for i in 0..max_len {
                let original_char = original_chars.get(i).copied();
                let normalized_char = normalized_chars.get(i).copied();

                if original_char != normalized_char {
                    if let (Some(orig), Some(norm)) = (original_char, normalized_char) {
                        changes.push(TextChange {
                            position: i,
                            original_char: orig,
                            normalized_char: norm,
                            change_type: ChangeType::UnicodeNormalization,
                            reason: format!("Unicode normalization {} → {}", orig, norm),
                        });
                    }
                }
            }
            changes
        } else {
            Vec::new()
        };

        NormalizedText {
            original: text.to_string(),
            normalized,
            changes,
            detected_script: crate::types::Script::Auto,
            processing_time_ms: 0,
        }
    }
}

impl Default for UnicodeNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfc_normalization() {
        let normalizer = UnicodeNormalizer::new();
        let result = normalizer.normalize("e\u{0301}", UnicodeNormalization::NFC);

        assert_eq!(result.normalized, "é");
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_nfd_normalization() {
        let normalizer = UnicodeNormalizer::new();
        let result = normalizer.normalize("é", UnicodeNormalization::NFD);

        assert_eq!(result.normalized, "e\u{0301}");
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_no_normalization() {
        let normalizer = UnicodeNormalizer::new();
        let result = normalizer.normalize("test", UnicodeNormalization::None);

        assert_eq!(result.normalized, "test");
        assert!(result.changes.is_empty());
    }
}
