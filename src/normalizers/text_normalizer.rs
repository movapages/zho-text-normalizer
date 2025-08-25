//! Main text normalizer that orchestrates all normalization steps

use crate::normalizers::{
    compatibility_normalizer::CompatibilityNormalizer, kangxi_normalizer::KangxiNormalizer,
    script_converter::ScriptConverter, script_detector::ScriptDetector,
    unicode_normalizer::UnicodeNormalizer, variant_normalizer::VariantNormalizer,
};
use crate::types::{NormalizedText, Script, UnicodeNormalization};
use std::time::Instant;

/// Main text normalizer that orchestrates all normalization steps
pub struct TextNormalizer {
    script_detector: ScriptDetector,
    script_converter: ScriptConverter,
    kangxi_normalizer: KangxiNormalizer,
    variant_normalizer: VariantNormalizer,
    compatibility_normalizer: CompatibilityNormalizer,
    unicode_normalizer: UnicodeNormalizer,
}

impl TextNormalizer {
    /// Create a new text normalizer
    pub fn new() -> Self {
        Self {
            script_detector: ScriptDetector::new(),
            script_converter: ScriptConverter::new(),
            kangxi_normalizer: KangxiNormalizer::new(),
            variant_normalizer: VariantNormalizer::new(),
            compatibility_normalizer: CompatibilityNormalizer::new(),
            unicode_normalizer: UnicodeNormalizer::new(),
        }
    }

    /// Normalize text with the specified target script
    pub fn normalize(&self, text: &str, target_script: Option<Script>) -> NormalizedText {
        let start_time = Instant::now();

        // Step 1: Detect script
        let detected_script = self.script_detector.detect(text);

        // Step 2: Unicode normalization (NFC)
        let unicode_result = self
            .unicode_normalizer
            .normalize(text, UnicodeNormalization::NFC);
        let mut all_changes = unicode_result.changes;

        // Step 3: Kangxi radical normalization
        let kangxi_result = self.kangxi_normalizer.normalize(&unicode_result.normalized);
        all_changes.extend(kangxi_result.changes);

        // Step 4: Character variant normalization
        let variant_result = self.variant_normalizer.normalize(&kangxi_result.normalized);
        all_changes.extend(variant_result.changes);

        // Step 5: Compatibility form normalization
        let compatibility_result = self
            .compatibility_normalizer
            .normalize(&variant_result.normalized);
        all_changes.extend(compatibility_result.changes);

        // Step 6: Script conversion (if target script is specified and different from detected)
        let final_text = if let Some(target) = target_script {
            if detected_script != target {
                let (converted_text, script_changes) = self.script_converter.convert(
                    &compatibility_result.normalized,
                    target,
                    detected_script.clone(),
                );
                all_changes.extend(script_changes);
                converted_text
            } else {
                compatibility_result.normalized
            }
        } else {
            compatibility_result.normalized
        };

        let processing_time = start_time.elapsed().as_millis() as u64;

        NormalizedText {
            original: text.to_string(),
            normalized: final_text,
            changes: all_changes,
            detected_script,
            processing_time_ms: processing_time,
        }
    }

    /// Validate text without performing conversions (for analysis)
    pub fn validate(&self, text: &str) -> NormalizedText {
        let start_time = Instant::now();

        // Step 1: Detect script
        let detected_script = self.script_detector.detect(text);

        // Step 2: Unicode normalization (NFC)
        let unicode_result = self
            .unicode_normalizer
            .normalize(text, UnicodeNormalization::NFC);
        let mut all_changes = unicode_result.changes;

        // Step 3: Kangxi radical normalization (validation only)
        let kangxi_result = self.kangxi_normalizer.normalize(&unicode_result.normalized);
        all_changes.extend(kangxi_result.changes);

        // Step 4: Character variant normalization (validation only)
        let variant_result = self.variant_normalizer.normalize(&kangxi_result.normalized);
        all_changes.extend(variant_result.changes);

        // Step 5: Compatibility form normalization (validation only)
        let compatibility_result = self
            .compatibility_normalizer
            .normalize(&variant_result.normalized);
        all_changes.extend(compatibility_result.changes);

        // No script conversion in validation mode
        let final_text = compatibility_result.normalized;

        let processing_time = start_time.elapsed().as_millis() as u64;

        NormalizedText {
            original: text.to_string(),
            normalized: final_text,
            changes: all_changes,
            detected_script,
            processing_time_ms: processing_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_normalization() {
        let normalizer = TextNormalizer::new();
        let result = normalizer.normalize("⽅⾯問題", None);

        assert_eq!(result.normalized, "方面問題");
        assert!(!result.changes.is_empty());
        assert!(result.processing_time_ms < 100); // Should be very fast
    }

    #[test]
    fn test_variant_normalization() {
        let normalizer = TextNormalizer::new();
        let result = normalizer.normalize("硏究敎育", None);

        assert_eq!(result.normalized, "硏究教育"); // Updated for kIICore algorithm
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_script_detection() {
        let normalizer = TextNormalizer::new();

        let chinese_result = normalizer.normalize("这是中文", None);
        assert!(matches!(
            chinese_result.detected_script,
            Script::SimplifiedChinese
        ));

        let japanese_result = normalizer.normalize("これは日本語です", None);
        assert!(matches!(japanese_result.detected_script, Script::Japanese));
    }

    #[test]
    fn test_kangxi_normalization() {
        let normalizer = TextNormalizer::new();
        let result = normalizer.normalize("⽅⾯問題", None);

        // Should normalize Kangxi radicals
        assert_eq!(result.normalized, "方面問題");
        assert!(!result.changes.is_empty());
    }
}
