//! Shared constants for file paths and configuration

/// Data file paths to avoid repetition across the codebase
pub mod paths {
    pub const SCRIPT_DIR: &str = "data/processed/script_conversion";
    pub const NORM_DIR: &str = "data/processed/normalization";

    // Script conversion files
    pub const T2S_MAPPINGS: &str =
        "data/processed/script_conversion/traditional_to_simplified.json";
    pub const S2T_MAPPINGS: &str =
        "data/processed/script_conversion/simplified_to_traditional.json";
    pub const SCRIPT_STATS: &str = "data/processed/script_conversion/script_conversion_stats.json";

    // Normalization files
    pub const SEMANTIC_VARIANTS: &str = "data/processed/normalization/semantic_variants.json";
    pub const COMPAT_VARIANTS: &str = "data/processed/normalization/compatibility_variants.json";
    pub const KANGXI_RADICALS: &str = "data/processed/normalization/kangxi_radicals.json";
    pub const NORM_STATS: &str = "data/processed/normalization/normalization_stats.json";

    // Source files
    pub const UNIHAN_IRG: &str = "Unihan/Unihan_IRGSources.txt";
    pub const UNIHAN_VARIANTS: &str = "Unihan/Unihan_Variants.txt";
}

/// Algorithm configuration constants
pub mod config {
    /// Minimum kIICore region count to consider a character "standard"
    pub const MIN_IICORE_REGIONS: usize = 4;

    /// Main CJK Unified Ideographs block range
    pub const MAIN_CJK_START: u32 = 0x4E00;
    pub const MAIN_CJK_END: u32 = 0x9FFF;

    /// Confidence thresholds for variant mappings
    pub const HIGH_CONFIDENCE: f64 = 0.9;
    pub const MEDIUM_CONFIDENCE: f64 = 0.8;
}
