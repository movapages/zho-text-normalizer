//! Advanced usage example for the ZHO Text Normalizer
//!
//! This example demonstrates advanced features including individual normalizer
//! components, statistics, and batch processing.

use zho_text_normalizer::normalizers::{
    CompatibilityNormalizer, KangxiNormalizer, VariantNormalizer,
};
use zho_text_normalizer::{Script, TextNormalizer};

fn main() {
    println!("=== Advanced ZHO Text Normalizer Usage ===\n");

    // Example 1: Using individual normalizers
    println!("1. Individual Normalizer Components");
    println!("-----------------------------------");

    let variant_normalizer = VariantNormalizer::new();
    let kangxi_normalizer = KangxiNormalizer::new();
    let compatibility_normalizer = CompatibilityNormalizer::new();

    // Get statistics about loaded mappings
    let stats = variant_normalizer.get_statistics();
    println!("Variant mapping statistics:");
    println!("  • Semantic variants: {}", stats.semantic_mappings);
    println!("  • Spoofing variants: {}", stats.spoofing_mappings);
    println!("  • Z-variants: {}", stats.z_variant_mappings);
    println!("  • Specialized variants: {}", stats.specialized_mappings);
    println!(
        "  • High confidence mappings: {}",
        stats.high_confidence_mappings
    );
    println!(
        "  • Bidirectional mappings: {}",
        stats.bidirectional_mappings
    );

    println!();

    // Example 2: Step-by-step normalization
    println!("2. Step-by-Step Normalization Process");
    println!("-------------------------------------");

    let input = "這個藥⼀天喫四回"; // Mixed: Traditional + Kangxi radical + variant
    println!("Original text: {}", input);

    // Step 1: Kangxi normalization
    let kangxi_result = kangxi_normalizer.normalize(input);
    println!(
        "After Kangxi:  {} (changes: {})",
        kangxi_result.normalized,
        kangxi_result.changes.len()
    );

    // Step 2: Variant normalization
    let variant_result = variant_normalizer.normalize(&kangxi_result.normalized);
    println!(
        "After variant: {} (changes: {})",
        variant_result.normalized,
        variant_result.changes.len()
    );

    // Step 3: Script conversion (using full normalizer for final step)
    let full_normalizer = TextNormalizer::new();
    let script_result =
        full_normalizer.normalize(&variant_result.normalized, Some(Script::SimplifiedChinese));
    println!(
        "After script:  {} (changes: {})",
        script_result.normalized,
        script_result.changes.len()
    );

    println!();

    // Example 3: Batch processing
    println!("3. Batch Processing");
    println!("------------------");

    let normalizer = TextNormalizer::new();
    let test_texts = vec!["傳統中文", "简体中文", "⽅⾔文字", "呌喫飯", "國際標準"];

    for (i, text) in test_texts.iter().enumerate() {
        let result = normalizer.normalize(text, Some(Script::SimplifiedChinese));
        println!(
            "  {}. {} → {} ({}ms, {} changes)",
            i + 1,
            text,
            result.normalized,
            result.processing_time_ms,
            result.changes.len()
        );
    }

    println!();

    // Example 4: Validation mode (no script conversion)
    println!("4. Validation Mode (Analysis Only)");
    println!("----------------------------------");

    let validation_text = "這是傳統⽂字範例"; // Traditional with Kangxi
    let validation_result = normalizer.validate(validation_text);

    println!("Input: {}", validation_result.original);
    println!("Detected script: {:?}", validation_result.detected_script);
    println!(
        "Potential normalizations found: {}",
        validation_result.changes.len()
    );

    if !validation_result.changes.is_empty() {
        println!("Analysis results:");
        for change in &validation_result.changes {
            println!(
                "  • {} → {} ({})",
                change.original_char, change.normalized_char, change.reason
            );
        }
    }

    println!();

    // Example 5: Working with specific variant types
    println!("5. Exploring Specific Variant Types");
    println!("-----------------------------------");

    let test_char = '呌'; // Semantic variant of 叫
    if let Some(mappings) = variant_normalizer.get_all_mappings(test_char) {
        println!(
            "Character '{}' has {} variant mapping(s):",
            test_char,
            mappings.len()
        );
        for mapping in mappings {
            println!(
                "  → '{}' (type: {:?}, confidence: {:.2})",
                mapping.target, mapping.variant_type, mapping.confidence
            );
            if !mapping.source_info.is_empty() {
                println!("    Sources: {}", mapping.source_info);
            }
        }
    } else {
        println!("No variant mappings found for '{}'", test_char);
    }
}
