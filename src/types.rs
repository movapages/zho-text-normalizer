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

/// Types of text changes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    ScriptConversion,
    KangxiRadical,
    VariantForm,
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

/// Script mapping with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMapping {
    pub traditional: String,
    pub simplified: String,
    pub pinyin: String,
    pub zhuyin: String,
    pub frequency: u32,
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
