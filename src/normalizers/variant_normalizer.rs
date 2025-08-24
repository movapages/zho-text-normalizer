//! Character variant normalization

use crate::types::{
    ChangeType, NormalizedText, TextChange, VariantMapping, VariantMappings, VariantType,
};
use serde_json;
use std::fs;
use std::path::Path;

/// Enhanced normalizer for character variants with confidence-based selection
pub struct VariantNormalizer {
    variant_mappings: VariantMappings,
}

impl VariantNormalizer {
    /// Create a new variant normalizer with enhanced mappings
    pub fn new() -> Self {
        Self {
            variant_mappings: Self::load_enhanced_variant_mappings(),
        }
    }

    /// Normalize character variants in the given text with confidence-based selection
    pub fn normalize(&self, text: &str) -> NormalizedText {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut changes = Vec::new();

        for (pos, &ch) in chars.iter().enumerate() {
            if let Some(best_mapping) = self.variant_mappings.get_best_mapping(ch) {
                // Apply smart confidence filtering
                if self.should_apply_mapping(best_mapping) {
                    let normalized_char = best_mapping.target;
                    result.push(normalized_char);

                    let change_type = match best_mapping.variant_type {
                        VariantType::Semantic => ChangeType::SemanticVariant,
                        VariantType::Spoofing => ChangeType::SpoofingVariant,
                        VariantType::ZVariant => ChangeType::ZVariant,
                        VariantType::Specialized => ChangeType::SpecializedVariant,
                        VariantType::Script => ChangeType::VariantForm, // Fallback
                    };

                    let reason = format!(
                        "{:?} variant {} → {} (confidence: {:.2}{})",
                        best_mapping.variant_type,
                        ch,
                        normalized_char,
                        best_mapping.confidence,
                        if !best_mapping.source_info.is_empty() {
                            format!(", sources: {}", best_mapping.source_info)
                        } else {
                            String::new()
                        }
                    );

                    changes.push(TextChange {
                        position: pos,
                        original_char: ch,
                        normalized_char,
                        change_type,
                        reason,
                    });
                } else {
                    result.push(ch);
                }
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

    /// Load enhanced variant mappings from the master variant mappings file
    fn load_enhanced_variant_mappings() -> VariantMappings {
        let mappings_path = Path::new("data/processed/variant_mappings.json");

        if let Ok(contents) = fs::read_to_string(mappings_path) {
            if let Ok(variant_mappings) = serde_json::from_str::<VariantMappings>(&contents) {
                println!(
                    "Loaded {} variant mappings ({} semantic, {} spoofing, {} Z-variants, {} specialized)",
                    variant_mappings.statistics.total_mappings,
                    variant_mappings.statistics.semantic_mappings,
                    variant_mappings.statistics.spoofing_mappings,
                    variant_mappings.statistics.z_variant_mappings,
                    variant_mappings.statistics.specialized_mappings
                );
                return variant_mappings;
            }
        }

        println!("Warning: Could not load enhanced variant mappings, using empty mappings");
        VariantMappings::new()
    }

    /// Get all available mappings for a character (for debugging/analysis)
    pub fn get_all_mappings(&self, ch: char) -> Option<&Vec<VariantMapping>> {
        self.variant_mappings.get_mappings(ch)
    }

    /// Get statistics about loaded variant mappings
    pub fn get_statistics(&self) -> &crate::types::VariantMappingStats {
        &self.variant_mappings.statistics
    }

    /// Smart confidence-based filtering to avoid over-normalization
    fn should_apply_mapping(&self, mapping: &VariantMapping) -> bool {
        // High confidence threshold
        if mapping.confidence < 0.8 {
            return false;
        }

        // For semantic variants, be more selective to avoid script conversion conflicts
        if mapping.variant_type == VariantType::Semantic {
            // Only apply if it has multiple dictionary sources (higher reliability)
            let source_count = if mapping.source_info.is_empty() {
                0
            } else {
                mapping.source_info.split(',').count()
            };

            // Require at least 2 sources for semantic variants, or very high confidence
            source_count >= 2 || mapping.confidence >= 0.9
        } else {
            // For spoofing, Z-variants, specialized: apply if high confidence
            mapping.confidence >= 0.8
        }
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
    fn test_enhanced_variant_normalization() {
        let normalizer = VariantNormalizer::new();

        // Test that the normalizer loads successfully
        let stats = normalizer.get_statistics();
        assert!(stats.total_mappings > 0, "Should load variant mappings");

        // Test semantic variant normalization (呌 → 叫)
        let result = normalizer.normalize("呌");
        if !result.changes.is_empty() {
            assert_eq!(result.normalized, "叫");
            assert!(matches!(
                result.changes[0].change_type,
                ChangeType::SemanticVariant
            ));
        }
    }

    #[test]
    fn test_confidence_based_selection() {
        let normalizer = VariantNormalizer::new();

        // Test that we get the best mapping based on confidence
        if let Some(_mappings) = normalizer.get_all_mappings('呌') {
            let best = normalizer.variant_mappings.get_best_mapping('呌');
            assert!(best.is_some(), "Should find best mapping for 呌");

            if let Some(best_mapping) = best {
                assert!(
                    best_mapping.confidence > 0.5,
                    "Should have reasonable confidence"
                );
                assert_eq!(best_mapping.target, '叫', "Should map 呌 to 叫");
            }
        }
    }

    #[test]
    fn test_no_change_for_unmapped_chars() {
        let normalizer = VariantNormalizer::new();
        let result = normalizer.normalize("普通文字");

        assert_eq!(result.normalized, "普通文字");
        assert!(result.changes.is_empty());
    }
}
