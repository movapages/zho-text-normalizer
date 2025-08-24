//! Basic usage example for the ZHO Text Normalizer
//!
//! This example demonstrates the core functionality of the normalizer,
//! including script conversion, variant normalization, and change tracking.

use zho_text_normalizer::{Script, TextNormalizer};

fn main() {
    // Initialize the normalizer
    let normalizer = TextNormalizer::new();

    // Example 1: Traditional to Simplified conversion
    println!("=== Traditional to Simplified ===");
    let traditional_text = "這個藥一天喫四回";
    let result = normalizer.normalize(traditional_text, Some(Script::SimplifiedChinese));

    println!("Original:   {}", result.original);
    println!("Normalized: {}", result.normalized);
    println!("Detected:   {:?}", result.detected_script);
    println!("Changes:    {} transformations", result.changes.len());

    // Show detailed changes
    for change in &result.changes {
        println!(
            "  {} → {} ({})",
            change.original_char,
            change.normalized_char,
            format!("{:?}", change.change_type)
        );
    }

    println!();

    // Example 2: Character variant normalization
    println!("=== Character Variant Normalization ===");
    let variant_text = "呌叫"; // 呌 is a semantic variant of 叫
    let result = normalizer.normalize(variant_text, None);

    println!("Original:   {}", result.original);
    println!("Normalized: {}", result.normalized);

    if !result.changes.is_empty() {
        println!("Changes:");
        for change in &result.changes {
            println!("  {}", change.reason);
        }
    }

    println!();

    // Example 3: Kangxi radical normalization
    println!("=== Kangxi Radical Normalization ===");
    let kangxi_text = "⽅⾯問題"; // Contains Kangxi radicals
    let result = normalizer.normalize(kangxi_text, None);

    println!("Original:   {}", result.original);
    println!("Normalized: {}", result.normalized);
    println!("Processing time: {}ms", result.processing_time_ms);

    println!();

    // Example 4: Mixed content normalization
    println!("=== Mixed Content Normalization ===");
    let mixed_text = "傳統⽂字與現代⽂本"; // Traditional + Kangxi radicals
    let result = normalizer.normalize(mixed_text, Some(Script::SimplifiedChinese));

    println!("Original:   {}", result.original);
    println!("Normalized: {}", result.normalized);
    println!("Script:     {:?}", result.detected_script);

    if !result.changes.is_empty() {
        println!("All changes:");
        for (i, change) in result.changes.iter().enumerate() {
            println!(
                "  {}. {} → {} at position {} ({})",
                i + 1,
                change.original_char,
                change.normalized_char,
                change.position,
                format!("{:?}", change.change_type)
            );
        }
    }
}
