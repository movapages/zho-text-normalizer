//! Script conversion (Traditional ↔ Simplified Chinese)

use crate::types::{ChangeType, Script, ScriptMapping, TextChange};
use crate::utils::opencc_validator::OpenCCValidator;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Converter for Traditional ↔ Simplified Chinese script conversion
pub struct ScriptConverter {
    traditional_to_simplified: HashMap<String, Vec<ScriptMapping>>,
    simplified_to_traditional: HashMap<String, Vec<ScriptMapping>>,
    opencc_validator: Option<OpenCCValidator>,
}

impl ScriptConverter {
    /// Create a new script converter
    pub fn new() -> Self {
        let (traditional_to_simplified, simplified_to_traditional) =
            Self::load_comprehensive_mappings();

        // Try to initialize OpenCC validator
        let opencc_validator = OpenCCValidator::new().ok();

        Self {
            traditional_to_simplified,
            simplified_to_traditional,
            opencc_validator,
        }
    }

    /// Convert text between Traditional and Simplified Chinese
    pub fn convert(
        &self,
        text: &str,
        target_script: Script,
        detected_script: Script,
    ) -> (String, Vec<TextChange>) {
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut changes = Vec::new();

        for (pos, &ch) in chars.iter().enumerate() {
            let converted_char = match (detected_script.clone(), target_script.clone()) {
                (Script::TraditionalChinese, Script::SimplifiedChinese) => {
                    self.convert_to_simplified(ch, pos, &mut changes)
                }
                (Script::SimplifiedChinese, Script::TraditionalChinese) => {
                    self.convert_to_traditional(ch, pos, &mut changes)
                }
                _ => ch, // No conversion needed
            };
            result.push(converted_char);
        }

        (result, changes)
    }

    /// Convert a character to Simplified Chinese
    fn convert_to_simplified(&self, ch: char, pos: usize, changes: &mut Vec<TextChange>) -> char {
        // First try OpenCC if available
        if let Some(ref opencc) = self.opencc_validator {
            if let Ok(converted) = opencc.traditional_to_simplified(&ch.to_string()) {
                if let Some(simp_char) = converted.chars().next() {
                    if simp_char != ch {
                        changes.push(TextChange {
                            position: pos,
                            original_char: ch,
                            normalized_char: simp_char,
                            change_type: ChangeType::ScriptConversion,
                            reason: format!(
                                "Traditional {} → Simplified {} (OpenCC)",
                                ch, simp_char
                            ),
                        });
                        return simp_char;
                    }
                }
            }
        }

        // Fallback to Unihan data
        if let Some(mappings) = self.traditional_to_simplified.get(&ch.to_string()) {
            if let Some(mapping) = mappings.first() {
                let simp_char = mapping.simplified.chars().next().unwrap_or(ch);
                if simp_char != ch {
                    changes.push(TextChange {
                        position: pos,
                        original_char: ch,
                        normalized_char: simp_char,
                        change_type: ChangeType::ScriptConversion,
                        reason: format!("Traditional {} → Simplified {} (Unihan)", ch, simp_char),
                    });
                    return simp_char;
                }
            }
        }

        ch // No conversion
    }

    /// Convert a character to Traditional Chinese
    fn convert_to_traditional(&self, ch: char, pos: usize, changes: &mut Vec<TextChange>) -> char {
        // First try OpenCC if available
        if let Some(ref opencc) = self.opencc_validator {
            if let Ok(converted) = opencc.simplified_to_traditional(&ch.to_string()) {
                if let Some(trad_char) = converted.chars().next() {
                    if trad_char != ch {
                        changes.push(TextChange {
                            position: pos,
                            original_char: ch,
                            normalized_char: trad_char,
                            change_type: ChangeType::ScriptConversion,
                            reason: format!(
                                "Simplified {} → Traditional {} (OpenCC)",
                                ch, trad_char
                            ),
                        });
                        return trad_char;
                    }
                }
            }
        }

        // Fallback to Unihan data
        if let Some(mappings) = self.simplified_to_traditional.get(&ch.to_string()) {
            if let Some(mapping) = mappings.first() {
                let trad_char = mapping.traditional.chars().next().unwrap_or(ch);
                if trad_char != ch {
                    changes.push(TextChange {
                        position: pos,
                        original_char: ch,
                        normalized_char: trad_char,
                        change_type: ChangeType::ScriptConversion,
                        reason: format!("Simplified {} → Traditional {} (Unihan)", ch, trad_char),
                    });
                    return trad_char;
                }
            }
        }

        ch // No conversion
    }

    /// Load comprehensive mappings from the new clean data structure
    fn load_comprehensive_mappings() -> (
        HashMap<String, Vec<ScriptMapping>>,
        HashMap<String, Vec<ScriptMapping>>,
    ) {
        let mut traditional_to_simplified = HashMap::new();
        let mut simplified_to_traditional = HashMap::new();

        // Load Traditional → Simplified mappings
        let t2s_path = Path::new("data/processed/script_conversion/traditional_to_simplified.json");
        if let Ok(contents) = fs::read_to_string(t2s_path) {
            if let Ok(t2s_mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (trad, simp) in t2s_mappings {
                    let mapping = ScriptMapping {
                        traditional: trad.clone(),
                        simplified: simp.clone(),
                        pinyin: String::new(),
                        zhuyin: String::new(),
                        frequency: 1,
                    };
                    traditional_to_simplified
                        .entry(trad)
                        .or_insert_with(Vec::new)
                        .push(mapping);
                }
            }
        }

        // Load Simplified → Traditional mappings
        let s2t_path = Path::new("data/processed/script_conversion/simplified_to_traditional.json");
        if let Ok(contents) = fs::read_to_string(s2t_path) {
            if let Ok(s2t_mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (simp, trad) in s2t_mappings {
                    let mapping = ScriptMapping {
                        traditional: trad.clone(),
                        simplified: simp.clone(),
                        pinyin: String::new(),
                        zhuyin: String::new(),
                        frequency: 1,
                    };
                    simplified_to_traditional
                        .entry(simp)
                        .or_insert_with(Vec::new)
                        .push(mapping);
                }
            }
        }

        // Fallback to hardcoded mappings if file doesn't exist
        if traditional_to_simplified.is_empty() {
            println!("Warning: No script mappings found, using fallback mappings");
            // Add some basic fallback mappings
            let fallback_mappings = vec![
                ("書".to_string(), "书".to_string()),
                ("說".to_string(), "说".to_string()),
                ("這".to_string(), "这".to_string()),
                ("個".to_string(), "个".to_string()),
                ("為".to_string(), "为".to_string()),
                ("國".to_string(), "国".to_string()),
                ("語".to_string(), "语".to_string()),
                ("學".to_string(), "学".to_string()),
                ("員".to_string(), "员".to_string()),
                ("參".to_string(), "参".to_string()),
            ];

            for (trad, simp) in fallback_mappings {
                let mapping = ScriptMapping {
                    traditional: trad.clone(),
                    simplified: simp.clone(),
                    pinyin: String::new(),
                    zhuyin: String::new(),
                    frequency: 1,
                };

                traditional_to_simplified
                    .entry(trad)
                    .or_insert_with(Vec::new)
                    .push(mapping.clone());

                simplified_to_traditional
                    .entry(simp)
                    .or_insert_with(Vec::new)
                    .push(mapping);
            }
        }

        println!(
            "Loaded {} comprehensive script mappings (traditional->simplified: {}, simplified->traditional: {})",
            traditional_to_simplified.len() + simplified_to_traditional.len(),
            traditional_to_simplified.len(),
            simplified_to_traditional.len()
        );

        (traditional_to_simplified, simplified_to_traditional)
    }
}

impl Default for ScriptConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traditional_to_simplified() {
        let converter = ScriptConverter::new();
        let (result, changes) = converter.convert(
            "榮耀歸於烏克蘭",
            Script::SimplifiedChinese,
            Script::TraditionalChinese,
        );

        assert!(!changes.is_empty());
        // OpenCC should convert most of these characters
        assert_ne!(result, "榮耀歸於烏克蘭");
    }

    #[test]
    fn test_simplified_to_traditional() {
        let converter = ScriptConverter::new();
        let (result, changes) = converter.convert(
            "荣耀归于乌克兰",
            Script::TraditionalChinese,
            Script::SimplifiedChinese,
        );

        assert!(!changes.is_empty());
        // OpenCC should convert most of these characters
        assert_ne!(result, "荣耀归于乌克兰");
    }

    #[test]
    fn test_no_conversion_needed() {
        let converter = ScriptConverter::new();
        let (result, changes) =
            converter.convert("test", Script::SimplifiedChinese, Script::SimplifiedChinese);

        assert_eq!(result, "test");
        assert!(changes.is_empty());
    }
}
