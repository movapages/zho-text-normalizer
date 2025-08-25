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

    // Check if any output files already exist (only check if not forcing)
    if !args.force {
        let script_dir = format!("{}/script_conversion", args.output_dir);
        let norm_dir = format!("{}/normalization", args.output_dir);

        if Path::new(&script_dir).exists() || Path::new(&norm_dir).exists() {
            println!("Output directories already exist. Use --force to reprocess the data.");
            return Ok(());
        }
    }

    // Process the Unihan data with clean separation
    println!("🚀 Processing Unihan data with clean separation...");
    UnihanDataProcessor::process_all()?;

    println!("\n✅ Processing complete! Check the generated files:");
    println!("  📁 Script conversion: data/processed/script_conversion/");
    println!("  📁 Normalization: data/processed/normalization/");

    Ok(())
}
