//! Core types and data structures for text normalization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Script types for CJK text
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Script {
    Auto,
    SimplifiedChinese,
    TraditionalChinese,
    Japanese,
    Korean,
}

/// Output format for the CLI
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Simple,
    Detailed,
    Verbose,
}

/// Unicode normalization forms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnicodeNormalization {
    None,
    NFC,
    NFD,
    NFKC,
    NFKD,
}

/// Types of character variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariantType {
    /// Semantic variants - characters with same meaning (呌 → 叫)
    Semantic,
    /// Spoofing variants - visually similar characters (security concern)
    Spoofing,
    /// Z-variants - different forms of the same character
    ZVariant,
    /// Specialized semantic variants - domain-specific variants
    Specialized,
    /// Traditional/Simplified script variants (handled separately)
    Script,
}

/// Types of text changes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    ScriptConversion,
    KangxiRadical,
    VariantForm,
    SemanticVariant,
    SpoofingVariant,
    ZVariant,
    SpecializedVariant,
    CompatibilityForm,
    UnicodeNormalization,
}

/// Individual text change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub position: usize,
    pub original_char: char,
    pub normalized_char: char,
    pub change_type: ChangeType,
    pub reason: String,
}

/// Enhanced variant mapping with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMapping {
    pub source: char,
    pub target: char,
    pub variant_type: VariantType,
    pub confidence: f32,
    pub bidirectional: bool,
    pub source_info: String, // Dictionary references (e.g., "kLau,kMatthews")
}

/// Script mapping with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMapping {
    pub traditional: String,
    pub simplified: String,
    pub pinyin: String,
    pub zhuyin: String,
    pub frequency: u32,
}

/// Statistics for variant mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMappingStats {
    pub total_mappings: usize,
    pub semantic_mappings: usize,
    pub spoofing_mappings: usize,
    pub z_variant_mappings: usize,
    pub specialized_mappings: usize,
    pub bidirectional_mappings: usize,
    pub high_confidence_mappings: usize,
}

/// Statistics for script mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMappingStats {
    pub total_mappings: usize,
    pub unique_traditional: usize,
    pub unique_simplified: usize,
    pub ambiguous_mappings: usize,
    pub single_character_mappings: usize,
    pub multi_character_mappings: usize,
}

/// Complete variant mappings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMappings {
    pub mappings: Vec<VariantMapping>,
    pub by_type: HashMap<VariantType, Vec<VariantMapping>>,
    pub lookup: HashMap<char, Vec<VariantMapping>>,
    pub statistics: VariantMappingStats,
}

/// Complete script mappings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMappings {
    pub traditional_to_simplified: HashMap<String, Vec<ScriptMapping>>,
    pub simplified_to_traditional: HashMap<String, Vec<ScriptMapping>>,
    pub statistics: ScriptMappingStats,
}

/// Normalized text result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedText {
    pub original: String,
    pub normalized: String,
    pub changes: Vec<TextChange>,
    pub detected_script: Script,
    pub processing_time_ms: u64,
}

/// Normalization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationConfig {
    pub target_script: Script,
    pub unicode_normalization: UnicodeNormalization,
    pub normalize_kangxi_radicals: bool,
    pub normalize_variants: bool,
    pub normalize_compatibility: bool,
    pub preserve_original: bool,
}

impl VariantMapping {
    /// Create a new variant mapping
    pub fn new(
        source: char,
        target: char,
        variant_type: VariantType,
        confidence: f32,
        bidirectional: bool,
        source_info: String,
    ) -> Self {
        Self {
            source,
            target,
            variant_type,
            confidence,
            bidirectional,
            source_info,
        }
    }

    /// Check if this mapping is high confidence (>= 0.8)
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.8
    }
}

impl VariantMappings {
    /// Create a new empty variant mappings structure
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            by_type: HashMap::new(),
            lookup: HashMap::new(),
            statistics: VariantMappingStats {
                total_mappings: 0,
                semantic_mappings: 0,
                spoofing_mappings: 0,
                z_variant_mappings: 0,
                specialized_mappings: 0,
                bidirectional_mappings: 0,
                high_confidence_mappings: 0,
            },
        }
    }

    /// Add a variant mapping and update indices
    pub fn add_mapping(&mut self, mapping: VariantMapping) {
        // Update statistics
        self.statistics.total_mappings += 1;
        match mapping.variant_type {
            VariantType::Semantic => self.statistics.semantic_mappings += 1,
            VariantType::Spoofing => self.statistics.spoofing_mappings += 1,
            VariantType::ZVariant => self.statistics.z_variant_mappings += 1,
            VariantType::Specialized => self.statistics.specialized_mappings += 1,
            VariantType::Script => {} // Handled separately
        }
        if mapping.bidirectional {
            self.statistics.bidirectional_mappings += 1;
        }
        if mapping.is_high_confidence() {
            self.statistics.high_confidence_mappings += 1;
        }

        // Add to by_type index
        self.by_type
            .entry(mapping.variant_type.clone())
            .or_insert_with(Vec::new)
            .push(mapping.clone());

        // Add to lookup index
        self.lookup
            .entry(mapping.source)
            .or_insert_with(Vec::new)
            .push(mapping.clone());

        // Add bidirectional mapping if specified
        if mapping.bidirectional {
            let reverse_mapping = VariantMapping {
                source: mapping.target,
                target: mapping.source,
                variant_type: mapping.variant_type.clone(),
                confidence: mapping.confidence,
                bidirectional: true,
                source_info: mapping.source_info.clone(),
            };

            self.lookup
                .entry(reverse_mapping.source)
                .or_insert_with(Vec::new)
                .push(reverse_mapping.clone());
        }

        // Add to main mappings
        self.mappings.push(mapping);
    }

    /// Get all mappings for a character
    pub fn get_mappings(&self, ch: char) -> Option<&Vec<VariantMapping>> {
        self.lookup.get(&ch)
    }

    /// Get best mapping for a character (highest confidence)
    pub fn get_best_mapping(&self, ch: char) -> Option<&VariantMapping> {
        self.get_mappings(ch)?.iter().max_by(|a, b| {
            a.confidence
                .partial_cmp(&b.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        Self {
            target_script: Script::Auto,
            unicode_normalization: UnicodeNormalization::NFC,
            normalize_kangxi_radicals: true,
            normalize_variants: true,
            normalize_compatibility: true,
            preserve_original: true,
        }
    }
}
