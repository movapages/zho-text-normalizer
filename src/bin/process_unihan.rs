//! Binary to process Unihan database files

use clap::Parser;
use std::path::Path;
use zho_text_normalizer::utils::UnihanDataProcessor;

#[derive(Parser)]
#[command(
    name = "process-unihan",
    about = "Process Unihan database files to generate mapping data",
    version
)]
struct Args {
    /// Output directory for processed files
    #[arg(short, long, default_value = "data/processed")]
    output_dir: String,

    /// Force reprocessing even if output files exist
    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Processing Unihan database files...");

    // Check if Unihan directory exists
    if !Path::new("Unihan").exists() {
        eprintln!("Error: Unihan directory not found!");
        eprintln!("Please download the Unihan database and place it in the 'Unihan' directory.");
        std::process::exit(1);
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)?;

    let output_path = format!("{}/script_mappings.json", args.output_dir);

    // Check if output file already exists
    if Path::new(&output_path).exists() && !args.force {
        println!("Output file already exists: {}", output_path);
        println!("Use --force to reprocess the data.");
        return Ok(());
    }

    // Process the Unihan data
    println!("Extracting Traditional ↔ Simplified mappings...");
    let mappings = UnihanDataProcessor::process_all()?;

    // Save the mappings
    println!("Saving mappings to: {}", output_path);
    UnihanDataProcessor::save_mappings(&mappings, &output_path)?;

    // Print statistics
    println!("\nProcessing complete!");
    println!("Statistics:");
    println!("  Total mappings: {}", mappings.statistics.total_mappings);
    println!(
        "  Unique traditional characters: {}",
        mappings.statistics.unique_traditional
    );
    println!(
        "  Unique simplified characters: {}",
        mappings.statistics.unique_simplified
    );
    println!(
        "  Ambiguous mappings: {}",
        mappings.statistics.ambiguous_mappings
    );
    println!(
        "  Single character mappings: {}",
        mappings.statistics.single_character_mappings
    );
    println!(
        "  Multi-character mappings: {}",
        mappings.statistics.multi_character_mappings
    );

    Ok(())
}
