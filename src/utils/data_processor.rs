//! Data processor for Unihan database files

use crate::types::{ScriptMapping, ScriptMappingStats, ScriptMappings};
use crate::utils::unicode_utils::code_point_to_char;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Processor for Unihan database files
pub struct UnihanDataProcessor;

impl UnihanDataProcessor {
    /// Process all Unihan files and generate mappings
    pub fn process_all() -> Result<ScriptMappings, Box<dyn std::error::Error>> {
        let processor = Self;

        // Process script mappings (Traditional ↔ Simplified)
        let script_variants = processor.process_script_variants("Unihan/Unihan_Variants.txt")?;

        // Process character form mappings (variants, compatibility, etc.)
        processor.process_character_form_mappings("Unihan/Unihan_Variants.txt")?;

        // Process Kangxi radical mappings
        processor.process_kangxi_mappings()?;

        // Combine script data into the final structure
        let mut traditional_to_simplified = HashMap::new();
        let mut simplified_to_traditional = HashMap::new();

        for (traditional, simplified, pinyin, zhuyin, frequency) in script_variants {
            let mapping = ScriptMapping {
                traditional,
                simplified,
                pinyin,
                zhuyin,
                frequency,
            };

            // Add to traditional to simplified mapping
            traditional_to_simplified
                .entry(mapping.traditional.clone())
                .or_insert_with(Vec::new)
                .push(mapping.clone());

            // Add to simplified to traditional mapping
            simplified_to_traditional
                .entry(mapping.simplified.clone())
                .or_insert_with(Vec::new)
                .push(mapping);
        }

        // Calculate statistics
        let stats =
            processor.calculate_stats(&traditional_to_simplified, &simplified_to_traditional);

        Ok(ScriptMappings {
            traditional_to_simplified,
            simplified_to_traditional,
            statistics: stats,
        })
    }

    /// Process script variants (Traditional ↔ Simplified) from kSimplifiedVariant and kTraditionalVariant
    fn process_script_variants(
        &self,
        path: &str,
    ) -> Result<Vec<(String, String, String, String, u32)>, Box<dyn std::error::Error>> {
        // Collect raw relationships
        let mut t2s_multi: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
        let mut s2t_multi: HashMap<String, std::collections::HashSet<String>> = HashMap::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 {
                continue;
            }

            let source_cp = parts[0];
            let kind = parts[1];
            let targets_raw = parts[2];

            let Some(source_char) = code_point_to_char(source_cp) else {
                continue;
            };

            match kind {
                "kSimplifiedVariant" => {
                    // A kSimplifiedVariant B => A (traditional) → B (simplified)
                    if let Some(first) = targets_raw.split_whitespace().next() {
                        if let Some(target_char) = code_point_to_char(first) {
                            t2s_multi
                                .entry(source_char.to_string())
                                .or_default()
                                .insert(target_char.to_string());
                        }
                    }
                }
                "kTraditionalVariant" => {
                    // A kTraditionalVariant B C ... => A (simplified) → {B, C, ...} (traditional)
                    for target_cp in targets_raw.split_whitespace() {
                        let clean = target_cp.split('<').next().unwrap_or(target_cp);
                        if let Some(target_char) = code_point_to_char(clean) {
                            s2t_multi
                                .entry(source_char.to_string())
                                .or_default()
                                .insert(target_char.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        // Build final mapping list with rules:
        // - Traditional→Simplified: allow many-to-one (collapse) using t2s_multi
        // - Simplified→Traditional: only one-to-one (unique) using s2t_multi and cross-check with t2s_multi
        // - Self-mappings will be handled by the converter when no mapping is found
        let mut final_mappings: Vec<(String, String, String, String, u32)> = Vec::new();

        // First, add all proper T→S mappings
        for (t, s_set) in &t2s_multi {
            for s in s_set {
                final_mappings.push((t.clone(), s.clone(), String::new(), String::new(), 1));
            }
        }

        // Then, add S→T mappings (only one-to-one)
        for (s, t_set) in &s2t_multi {
            if t_set.len() == 1 {
                let t = t_set.iter().next().unwrap().clone();
                // Ensure t→s exists (one-to-one consistency)
                if t2s_multi
                    .get(&t)
                    .map(|ss| ss.len() == 1 && ss.contains(s))
                    .unwrap_or(false)
                {
                    final_mappings.push((t, s.clone(), String::new(), String::new(), 1));
                }
            }
        }

        // Note: Self-mappings are handled by the converter when no explicit mapping is found
        // This prevents self-mappings from shadowing proper conversions

        Ok(final_mappings)
    }

    /// Process character form mappings (variants, compatibility, etc.)
    fn process_character_form_mappings(
        &self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut variant_mappings = HashMap::new();
        let mut compatibility_mappings = HashMap::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 {
                continue;
            }

            let source_cp = parts[0];
            let kind = parts[1];
            let targets_raw = parts[2];

            let Some(source_char) = code_point_to_char(source_cp) else {
                continue;
            };

            match kind {
                "kSemanticVariant" | "kZVariant" | "kSpecializedSemanticVariant" => {
                    // Handle semantic variants for character normalization
                    let targets: Vec<&str> = targets_raw.split_whitespace().collect();
                    if targets.len() == 1 {
                        let clean_target = targets[0].split('<').next().unwrap_or(targets[0]);
                        if let Some(target_char) = code_point_to_char(clean_target) {
                            let source_code = source_char as u32;
                            let target_code = target_char as u32;

                            if source_code >= 0x4E00
                                && source_code <= 0x9FFF
                                && target_code >= 0x4E00
                                && target_code <= 0x9FFF
                                && source_char != target_char
                            {
                                // Better heuristic: use character complexity
                                let source_complexity = Self::estimate_complexity(source_char);
                                let target_complexity = Self::estimate_complexity(target_char);

                                if source_complexity > target_complexity {
                                    variant_mappings
                                        .insert(source_char.to_string(), target_char.to_string());
                                } else if target_complexity > source_complexity {
                                    variant_mappings
                                        .insert(target_char.to_string(), source_char.to_string());
                                }
                                // If complexity is equal, skip to avoid ambiguity
                            }
                        }
                    }
                }
                "kCompatibilityVariant" => {
                    // Handle compatibility variants
                    let targets: Vec<&str> = targets_raw.split_whitespace().collect();
                    if targets.len() == 1 {
                        let clean_target = targets[0].split('<').next().unwrap_or(targets[0]);
                        if let Some(target_char) = code_point_to_char(clean_target) {
                            let source_code = source_char as u32;
                            let target_code = target_char as u32;

                            if source_code >= 0x4E00
                                && source_code <= 0x9FFF
                                && target_code >= 0x4E00
                                && target_code <= 0x9FFF
                                && source_char != target_char
                            {
                                // For compatibility variants, assume the compatibility form is the variant
                                compatibility_mappings
                                    .insert(source_char.to_string(), target_char.to_string());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Save variant mappings
        let variant_mappings_path = "data/processed/variant_mappings.json";
        let variant_mappings_json = serde_json::to_string_pretty(&variant_mappings)?;
        fs::write(variant_mappings_path, variant_mappings_json)?;
        println!(
            "Saved {} variant mappings to: {}",
            variant_mappings.len(),
            variant_mappings_path
        );

        // Save compatibility mappings
        let compatibility_mappings_path = "data/processed/compatibility_mappings.json";
        let compatibility_mappings_json = serde_json::to_string_pretty(&compatibility_mappings)?;
        fs::write(compatibility_mappings_path, compatibility_mappings_json)?;
        println!(
            "Saved {} compatibility mappings to: {}",
            compatibility_mappings.len(),
            compatibility_mappings_path
        );

        Ok(())
    }

    /// Process Kangxi radical mappings
    ///
    /// NOTE: These mappings are hardcoded (not extracted from Unihan) because:
    /// 1. Kangxi radicals are a fixed, standardized set (214 radicals) defined by Unicode
    /// 2. Each radical has exactly one canonical equivalent (1:1 mapping)
    /// 3. The mapping never changes (Unicode standard is stable)
    /// 4. No direct Unihan property exists for Kangxi → standard character mappings
    /// 5. While Unicode decomposition exists, hardcoding is simpler and more reliable
    fn process_kangxi_mappings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut kangxi_mappings = HashMap::new();

        // Kangxi Radicals (U+2F00-U+2FDF) to Standard Characters
        // This is a comprehensive mapping of all 214 Kangxi radicals
        let kangxi_data = [
            (0x2F00, '一'),
            (0x2F01, '丨'),
            (0x2F02, '丶'),
            (0x2F03, '丿'),
            (0x2F04, '乙'),
            (0x2F05, '亅'),
            (0x2F06, '二'),
            (0x2F07, '亠'),
            (0x2F08, '人'),
            (0x2F09, '儿'),
            (0x2F0A, '入'),
            (0x2F0B, '八'),
            (0x2F0C, '冂'),
            (0x2F0D, '冖'),
            (0x2F0E, '冫'),
            (0x2F0F, '几'),
            (0x2F10, '凵'),
            (0x2F11, '刀'),
            (0x2F12, '力'),
            (0x2F13, '勹'),
            (0x2F14, '匕'),
            (0x2F15, '匚'),
            (0x2F16, '匸'),
            (0x2F17, '十'),
            (0x2F18, '卜'),
            (0x2F19, '卩'),
            (0x2F1A, '厂'),
            (0x2F1B, '厶'),
            (0x2F1C, '又'),
            (0x2F1D, '口'),
            (0x2F1E, '囗'),
            (0x2F1F, '土'),
            (0x2F20, '士'),
            (0x2F21, '夂'),
            (0x2F22, '夊'),
            (0x2F23, '夕'),
            (0x2F24, '大'),
            (0x2F25, '女'),
            (0x2F26, '子'),
            (0x2F27, '宀'),
            (0x2F28, '寸'),
            (0x2F29, '小'),
            (0x2F2A, '尢'),
            (0x2F2B, '尸'),
            (0x2F2C, '屮'),
            (0x2F2D, '山'),
            (0x2F2E, '巛'),
            (0x2F2F, '工'),
            (0x2F30, '己'),
            (0x2F31, '巾'),
            (0x2F32, '干'),
            (0x2F33, '幺'),
            (0x2F34, '广'),
            (0x2F35, '廴'),
            (0x2F36, '廾'),
            (0x2F37, '弋'),
            (0x2F38, '弓'),
            (0x2F39, '彐'),
            (0x2F3A, '彡'),
            (0x2F3B, '彳'),
            (0x2F3C, '心'),
            (0x2F3D, '戈'),
            (0x2F3E, '戶'),
            (0x2F3F, '手'),
            (0x2F40, '支'),
            (0x2F41, '攴'),
            (0x2F42, '文'),
            (0x2F43, '斗'),
            (0x2F44, '斤'),
            (0x2F45, '方'),
            (0x2F46, '无'),
            (0x2F47, '日'),
            (0x2F48, '曰'),
            (0x2F49, '月'),
            (0x2F4A, '木'),
            (0x2F4B, '欠'),
            (0x2F4C, '止'),
            (0x2F4D, '歹'),
            (0x2F4E, '殳'),
            (0x2F4F, '毋'),
            (0x2F50, '比'),
            (0x2F51, '毛'),
            (0x2F52, '氏'),
            (0x2F53, '气'),
            (0x2F54, '水'),
            (0x2F55, '火'),
            (0x2F56, '爪'),
            (0x2F57, '父'),
            (0x2F58, '爻'),
            (0x2F59, '爿'),
            (0x2F5A, '片'),
            (0x2F5B, '牙'),
            (0x2F5C, '牛'),
            (0x2F5D, '犬'),
            (0x2F5E, '玄'),
            (0x2F5F, '玉'),
            (0x2F60, '瓜'),
            (0x2F61, '瓦'),
            (0x2F62, '甘'),
            (0x2F63, '生'),
            (0x2F64, '用'),
            (0x2F65, '田'),
            (0x2F66, '疋'),
            (0x2F67, '疒'),
            (0x2F68, '癶'),
            (0x2F69, '白'),
            (0x2F6A, '皮'),
            (0x2F6B, '皿'),
            (0x2F6C, '目'),
            (0x2F6D, '矛'),
            (0x2F6E, '矢'),
            (0x2F6F, '石'),
            (0x2F70, '示'),
            (0x2F71, '禸'),
            (0x2F72, '禾'),
            (0x2F73, '穴'),
            (0x2F74, '立'),
            (0x2F75, '竹'),
            (0x2F76, '米'),
            (0x2F77, '糸'),
            (0x2F78, '缶'),
            (0x2F79, '网'),
            (0x2F7A, '羊'),
            (0x2F7B, '羽'),
            (0x2F7C, '老'),
            (0x2F7D, '而'),
            (0x2F7E, '耒'),
            (0x2F7F, '耳'),
            (0x2F80, '聿'),
            (0x2F81, '肉'),
            (0x2F82, '臣'),
            (0x2F83, '自'),
            (0x2F84, '至'),
            (0x2F85, '臼'),
            (0x2F86, '舌'),
            (0x2F87, '舛'),
            (0x2F88, '舟'),
            (0x2F89, '艮'),
            (0x2F8A, '色'),
            (0x2F8B, '艸'),
            (0x2F8C, '虍'),
            (0x2F8D, '虫'),
            (0x2F8E, '血'),
            (0x2F8F, '行'),
            (0x2F90, '衣'),
            (0x2F91, '襾'),
            (0x2F92, '見'),
            (0x2F93, '角'),
            (0x2F94, '言'),
            (0x2F95, '谷'),
            (0x2F96, '豆'),
            (0x2F97, '豕'),
            (0x2F98, '豸'),
            (0x2F99, '貝'),
            (0x2F9A, '赤'),
            (0x2F9B, '走'),
            (0x2F9C, '足'),
            (0x2F9D, '身'),
            (0x2F9E, '車'),
            (0x2F9F, '辛'),
            (0x2FA0, '辰'),
            (0x2FA1, '辵'),
            (0x2FA2, '邑'),
            (0x2FA3, '酉'),
            (0x2FA4, '釆'),
            (0x2FA5, '里'),
            (0x2FA6, '金'),
            (0x2FA7, '長'),
            (0x2FA8, '門'),
            (0x2FA9, '阜'),
            (0x2FAA, '隶'),
            (0x2FAB, '隹'),
            (0x2FAC, '雨'),
            (0x2FAD, '青'),
            (0x2FAE, '非'),
            (0x2FAF, '面'),
            (0x2FB0, '革'),
            (0x2FB1, '韋'),
            (0x2FB2, '韭'),
            (0x2FB3, '音'),
            (0x2FB4, '頁'),
            (0x2FB5, '風'),
            (0x2FB6, '飛'),
            (0x2FB7, '食'),
            (0x2FB8, '首'),
            (0x2FB9, '香'),
            (0x2FBA, '馬'),
            (0x2FBB, '骨'),
            (0x2FBC, '高'),
            (0x2FBD, '髟'),
            (0x2FBE, '鬥'),
            (0x2FBF, '鬯'),
            (0x2FC0, '鬲'),
            (0x2FC1, '鬼'),
            (0x2FC2, '魚'),
            (0x2FC3, '鳥'),
            (0x2FC4, '鹵'),
            (0x2FC5, '鹿'),
            (0x2FC6, '麥'),
            (0x2FC7, '麻'),
            (0x2FC8, '黃'),
            (0x2FC9, '黍'),
            (0x2FCA, '黑'),
            (0x2FCB, '黹'),
            (0x2FCC, '黽'),
            (0x2FCD, '鼎'),
            (0x2FCE, '鼓'),
            (0x2FCF, '鼠'),
            (0x2FD0, '鼻'),
            (0x2FD1, '齊'),
            (0x2FD2, '齒'),
            (0x2FD3, '龍'),
            (0x2FD4, '龜'),
            (0x2FD5, '龠'),
        ];

        for (code_point, standard_char) in kangxi_data {
            if let Some(kangxi_char) = char::from_u32(code_point) {
                kangxi_mappings.insert(kangxi_char.to_string(), standard_char.to_string());
            }
        }

        // Save Kangxi mappings
        let kangxi_mappings_path = "data/processed/kangxi_mappings.json";
        let kangxi_mappings_json = serde_json::to_string_pretty(&kangxi_mappings)?;
        fs::write(kangxi_mappings_path, kangxi_mappings_json)?;
        println!(
            "Saved {} Kangxi mappings to: {}",
            kangxi_mappings.len(),
            kangxi_mappings_path
        );

        Ok(())
    }

    /// Estimate character complexity based on Unicode block and code point
    fn estimate_complexity(c: char) -> u32 {
        let code = c as u32;

        // Simple heuristic: characters in higher Unicode blocks tend to be more complex
        // This is a rough approximation - in a real system you'd use actual stroke count data
        if code >= 0x4E00 && code <= 0x9FFF {
            // Main CJK Unified Ideographs
            // Higher code points in this range tend to be more complex
            (code - 0x4E00) / 1000 + 1
        } else {
            1
        }
    }

    /// Calculate statistics about the mappings
    fn calculate_stats(
        &self,
        traditional_to_simplified: &HashMap<String, Vec<ScriptMapping>>,
        simplified_to_traditional: &HashMap<String, Vec<ScriptMapping>>,
    ) -> ScriptMappingStats {
        let total_mappings = traditional_to_simplified
            .values()
            .map(|v| v.len())
            .sum::<usize>();
        let unique_traditional = traditional_to_simplified.len();
        let unique_simplified = simplified_to_traditional.len();

        let ambiguous_mappings = traditional_to_simplified
            .values()
            .filter(|v| v.len() > 1)
            .map(|v| v.len())
            .sum::<usize>();

        let single_character_mappings = traditional_to_simplified
            .values()
            .filter(|v| {
                v.iter().all(|m| {
                    m.traditional.chars().count() == 1 && m.simplified.chars().count() == 1
                })
            })
            .map(|v| v.len())
            .sum::<usize>();

        let multi_character_mappings = total_mappings - single_character_mappings;

        ScriptMappingStats {
            total_mappings,
            unique_traditional,
            unique_simplified,
            ambiguous_mappings,
            single_character_mappings,
            multi_character_mappings,
        }
    }

    /// Save processed mappings to JSON file
    pub fn save_mappings(
        mappings: &ScriptMappings,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(mappings)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load mappings from JSON file
    pub fn load_mappings(path: &str) -> Result<ScriptMappings, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mappings: ScriptMappings = serde_json::from_reader(reader)?;
        Ok(mappings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants_processing() {
        // This test would require a small sample of the Unihan data
        // For now, we'll just test that the processor can be created
        let _processor = UnihanDataProcessor;
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_code_point_conversion() {
        assert_eq!(code_point_to_char("U+4E00"), Some('一'));
        assert_eq!(code_point_to_char("U+9FFF"), Some('鿿'));
    }
}
