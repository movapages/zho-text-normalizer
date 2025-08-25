//! Data processor for Unihan database files

// Note: ScriptMapping types removed as we now use simple HashMap<String, String> for clean data
use crate::utils::unicode_utils::code_point_to_char;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Data file path constants to avoid repetition
mod paths {
    pub const SCRIPT_DIR: &str = "data/processed/script_conversion";
    pub const NORM_DIR: &str = "data/processed/normalization";

    pub const T2S_MAPPINGS: &str =
        "data/processed/script_conversion/traditional_to_simplified.json";
    pub const S2T_MAPPINGS: &str =
        "data/processed/script_conversion/simplified_to_traditional.json";
    pub const SCRIPT_STATS: &str = "data/processed/script_conversion/script_conversion_stats.json";

    pub const SEMANTIC_VARIANTS: &str = "data/processed/normalization/semantic_variants.json";
    pub const COMPAT_VARIANTS: &str = "data/processed/normalization/compatibility_variants.json";
    pub const KANGXI_RADICALS: &str = "data/processed/normalization/kangxi_radicals.json";
    pub const NORM_STATS: &str = "data/processed/normalization/normalization_stats.json";

    pub const UNIHAN_IRG: &str = "Unihan/Unihan_IRGSources.txt";
}

/// Processor for Unihan database files
pub struct UnihanDataProcessor;

impl UnihanDataProcessor {
    /// Process all Unihan files and generate clean separated mappings
    pub fn process_all() -> Result<(), Box<dyn std::error::Error>> {
        let processor = Self;
        println!("🚀 Starting clean data generation with proper separation...");

        // Step 1: Process script conversion mappings (Traditional ↔ Simplified)
        println!("\n📋 Step 1: Processing script conversion mappings...");
        processor.process_script_conversion_mappings("Unihan/Unihan_Variants.txt")?;

        // Step 2: Process normalization mappings (variants, compatibility, etc.)
        // EXCLUDING pairs that already exist in script conversion
        println!("\n📋 Step 2: Processing normalization mappings...");
        processor.process_normalization_mappings(
            "Unihan/Unihan_Variants.txt",
            "Unihan/Unihan_IRGSources.txt",
        )?;

        println!("\n✅ Clean data generation completed!");
        Ok(())
    }

    /// Process script conversion mappings (Traditional ↔ Simplified) from kSimplifiedVariant and kTraditionalVariant
    fn process_script_conversion_mappings(
        &self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Collect raw relationships
        let mut t2s_mappings: HashMap<String, String> = HashMap::new();
        let mut s2t_mappings: HashMap<String, String> = HashMap::new();
        let mut processed_pairs: HashSet<(String, String)> = HashSet::new();

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
                            let trad = source_char.to_string();
                            let simp = target_char.to_string();

                            if !processed_pairs.contains(&(trad.clone(), simp.clone())) {
                                t2s_mappings.insert(trad.clone(), simp.clone());
                                processed_pairs.insert((trad, simp));
                            }
                        }
                    }
                }
                "kTraditionalVariant" => {
                    // A kTraditionalVariant B => A (simplified) → B (traditional)
                    // Only take the first one to avoid ambiguity
                    if let Some(first) = targets_raw.split_whitespace().next() {
                        let clean = first.split('<').next().unwrap_or(first);
                        if let Some(target_char) = code_point_to_char(clean) {
                            let simp = source_char.to_string();
                            let trad = target_char.to_string();

                            if !processed_pairs.contains(&(trad.clone(), simp.clone())) {
                                s2t_mappings.insert(simp.clone(), trad.clone());
                                processed_pairs.insert((trad, simp));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Save Traditional → Simplified mappings
        let t2s_path = paths::T2S_MAPPINGS;
        let t2s_json = serde_json::to_string_pretty(&t2s_mappings)?;
        fs::write(t2s_path, t2s_json)?;
        println!(
            "✅ Saved {} Traditional→Simplified mappings to: {}",
            t2s_mappings.len(),
            t2s_path
        );

        // Save Simplified → Traditional mappings
        let s2t_path = paths::S2T_MAPPINGS;
        let s2t_json = serde_json::to_string_pretty(&s2t_mappings)?;
        fs::write(s2t_path, s2t_json)?;
        println!(
            "✅ Saved {} Simplified→Traditional mappings to: {}",
            s2t_mappings.len(),
            s2t_path
        );

        // Save statistics
        let stats = serde_json::json!({
            "traditional_to_simplified_count": t2s_mappings.len(),
            "simplified_to_traditional_count": s2t_mappings.len(),
            "total_script_conversion_pairs": processed_pairs.len(),
            "generation_timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        });
        let stats_path = paths::SCRIPT_STATS;
        fs::write(stats_path, serde_json::to_string_pretty(&stats)?)?;
        println!("✅ Saved statistics to: {}", stats_path);

        Ok(())
    }

    /// Process normalization mappings (variants → standard forms) EXCLUDING script conversion pairs
    fn process_normalization_mappings(
        &self,
        variants_path: &str,
        irg_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Step 1: Load existing script conversion pairs to exclude them
        let script_pairs = self.load_script_conversion_pairs()?;
        println!(
            "📋 Loaded {} script conversion pairs to exclude",
            script_pairs.len()
        );

        // Step 2: Process semantic variants
        let semantic_variants =
            self.process_semantic_variants_clean(variants_path, &script_pairs)?;

        // Step 3: Process compatibility variants
        let compatibility_variants =
            self.process_compatibility_variants_clean(irg_path, &script_pairs)?;

        // Step 4: Process Kangxi radicals
        let kangxi_variants = self.process_kangxi_radicals_clean(&script_pairs)?;

        // Step 5: Save normalization statistics
        let stats = serde_json::json!({
            "semantic_variants_count": semantic_variants.len(),
            "compatibility_variants_count": compatibility_variants.len(),
            "kangxi_radicals_count": kangxi_variants.len(),
            "total_normalization_mappings": semantic_variants.len() + compatibility_variants.len() + kangxi_variants.len(),
            "excluded_script_pairs": script_pairs.len(),
            "generation_timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        });
        let stats_path = "data/processed/normalization/normalization_stats.json";
        fs::write(stats_path, serde_json::to_string_pretty(&stats)?)?;
        println!("✅ Saved normalization statistics to: {}", stats_path);

        Ok(())
    }

    /// Load script conversion pairs to exclude from normalization
    fn load_script_conversion_pairs(
        &self,
    ) -> Result<HashSet<(String, String)>, Box<dyn std::error::Error>> {
        let mut pairs = HashSet::new();

        // Load Traditional → Simplified
        if let Ok(contents) =
            fs::read_to_string("data/processed/script_conversion/traditional_to_simplified.json")
        {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (trad, simp) in mappings {
                    pairs.insert((trad, simp));
                }
            }
        }

        // Load Simplified → Traditional
        if let Ok(contents) =
            fs::read_to_string("data/processed/script_conversion/simplified_to_traditional.json")
        {
            if let Ok(mappings) = serde_json::from_str::<HashMap<String, String>>(&contents) {
                for (simp, trad) in mappings {
                    pairs.insert((trad, simp));
                }
            }
        }

        Ok(pairs)
    }

    /// Process semantic variants with proper standard form detection, excluding script pairs
    fn process_semantic_variants_clean(
        &self,
        path: &str,
        script_pairs: &HashSet<(String, String)>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut semantic_mappings = HashMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 || parts[1] != "kSemanticVariant" {
                continue;
            }

            let source_cp = parts[0];
            let targets_raw = parts[2];

            let Some(source_char) = code_point_to_char(source_cp) else {
                continue;
            };

            // Process each target
            for target_raw in targets_raw.split_whitespace() {
                let clean_target = target_raw.split('<').next().unwrap_or(target_raw);
                if let Some(target_char) = code_point_to_char(clean_target) {
                    let source_str = source_char.to_string();
                    let target_str = target_char.to_string();

                    // Skip if this pair is already handled by script conversion
                    if script_pairs.contains(&(source_str.clone(), target_str.clone()))
                        || script_pairs.contains(&(target_str.clone(), source_str.clone()))
                    {
                        continue;
                    }

                    // Determine standard form using Unicode block priority
                    if let Some((variant, standard)) =
                        self.determine_standard_form(source_char, target_char)
                    {
                        semantic_mappings.insert(variant.to_string(), standard.to_string());
                    }
                }
            }
        }

        // Save semantic variants
        let path = "data/processed/normalization/semantic_variants.json";
        let json = serde_json::to_string_pretty(&semantic_mappings)?;
        fs::write(path, json)?;
        println!(
            "✅ Saved {} semantic variant mappings to: {}",
            semantic_mappings.len(),
            path
        );

        Ok(semantic_mappings)
    }

    /// Determine which character is the standard form
    fn determine_standard_form(&self, char1: char, char2: char) -> Option<(char, char)> {
        let code1 = char1 as u32;
        let code2 = char2 as u32;

        // Primary rule: Main CJK block (U+4E00-U+9FFF) is preferred over compatibility blocks
        let is_main_1 = code1 >= 0x4E00 && code1 <= 0x9FFF;
        let is_main_2 = code2 >= 0x4E00 && code2 <= 0x9FFF;

        match (is_main_1, is_main_2) {
            (true, false) => Some((char2, char1)), // compatibility → main
            (false, true) => Some((char1, char2)), // compatibility → main
            (true, true) => {
                // Both in main CJK: use kIICore region count to determine standard form
                let iicore1 = self.get_iicore_count(char1);
                let iicore2 = self.get_iicore_count(char2);

                if iicore1 > iicore2 {
                    Some((char2, char1)) // variant → standard (char1 is more standard)
                } else if iicore2 > iicore1 {
                    Some((char1, char2)) // variant → standard (char2 is more standard)
                } else {
                    // Equal or no kIICore data - skip ambiguous cases
                    None
                }
            }
            (false, false) => None, // Both in compatibility blocks - skip
        }
    }

    /// Get kIICore region count for a character (higher count = more standard)
    fn get_iicore_count(&self, ch: char) -> usize {
        let code_point = format!("U+{:04X}", ch as u32);

        // Try to read from Unihan IRG Sources file
        if let Ok(contents) = fs::read_to_string("Unihan/Unihan_IRGSources.txt") {
            for line in contents.lines() {
                if line.starts_with(&code_point) && line.contains("kIICore") {
                    // Extract kIICore value: "U+4E00  kIICore AGTJHKMP"
                    if let Some(iicore_part) = line.split("kIICore").nth(1) {
                        let iicore_regions = iicore_part.trim();
                        return iicore_regions.len(); // Each letter = one region
                    }
                }
            }
        }

        0 // No kIICore data found
    }

    /// Process compatibility variants excluding script pairs
    fn process_compatibility_variants_clean(
        &self,
        path: &str,
        script_pairs: &HashSet<(String, String)>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut compatibility_mappings = HashMap::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 3 || parts[1] != "kCompatibilityVariant" {
                continue;
            }

            let source_cp = parts[0];
            let targets_raw = parts[2];

            let Some(source_char) = code_point_to_char(source_cp) else {
                continue;
            };

            if let Some(target_raw) = targets_raw.split_whitespace().next() {
                let clean_target = target_raw.split('<').next().unwrap_or(target_raw);
                if let Some(target_char) = code_point_to_char(clean_target) {
                    let source_str = source_char.to_string();
                    let target_str = target_char.to_string();

                    // Skip if this pair is already handled by script conversion
                    if script_pairs.contains(&(source_str.clone(), target_str.clone()))
                        || script_pairs.contains(&(target_str.clone(), source_str.clone()))
                    {
                        continue;
                    }

                    // For compatibility variants, source is always the variant
                    compatibility_mappings.insert(source_str, target_str);
                }
            }
        }

        // Save compatibility variants
        let path = "data/processed/normalization/compatibility_variants.json";
        let json = serde_json::to_string_pretty(&compatibility_mappings)?;
        fs::write(path, json)?;
        println!(
            "✅ Saved {} compatibility variant mappings to: {}",
            compatibility_mappings.len(),
            path
        );

        Ok(compatibility_mappings)
    }

    /// Process Kangxi radicals excluding script pairs
    fn process_kangxi_radicals_clean(
        &self,
        script_pairs: &HashSet<(String, String)>,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut kangxi_mappings = HashMap::new();

        // Hardcoded Kangxi radical mappings (these are fixed Unicode assignments)
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
                let kangxi_str = kangxi_char.to_string();
                let standard_str = standard_char.to_string();

                // Skip if this pair is already handled by script conversion
                if !script_pairs.contains(&(kangxi_str.clone(), standard_str.clone()))
                    && !script_pairs.contains(&(standard_str.clone(), kangxi_str.clone()))
                {
                    kangxi_mappings.insert(kangxi_str, standard_str);
                }
            }
        }

        // Save Kangxi mappings
        let path = "data/processed/normalization/kangxi_radicals.json";
        let json = serde_json::to_string_pretty(&kangxi_mappings)?;
        fs::write(path, json)?;
        println!(
            "✅ Saved {} Kangxi radical mappings to: {}",
            kangxi_mappings.len(),
            path
        );

        Ok(kangxi_mappings)
    }

    // === REMOVED LEGACY METHODS ===
    // The following methods were removed as they're replaced by the new clean separation approach:
    // - process_script_variants (replaced by process_script_conversion_mappings)
    // - process_character_form_mappings (replaced by process_normalization_mappings)
    // - process_kangxi_mappings (replaced by process_kangxi_radicals_clean)
    // - estimate_complexity (replaced by get_iicore_count)
    // - calculate_stats (replaced by inline statistics generation)
}
