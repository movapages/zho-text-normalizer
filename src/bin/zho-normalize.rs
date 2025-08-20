use clap::Parser;
use zho_text_normalizer::normalizers::text_normalizer::TextNormalizer;
use zho_text_normalizer::types::{OutputFormat, Script};

#[derive(Parser)]
#[command(name = "zho-normalize")]
#[command(about = "Chinese text normalizer")]
struct Args {
    /// Input text to normalize
    #[arg(value_name = "TEXT")]
    text: String,

    /// Target script for conversion (auto, simplified, traditional)
    #[arg(short, long, default_value = "auto")]
    target: String,

    /// Output format (simple, detailed, verbose)
    #[arg(short, long, default_value = "simple")]
    format: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Validation mode (no conversion, just analysis)
    #[arg(long)]
    validate: bool,
}

fn parse_script(script: &str) -> Script {
    match script.to_lowercase().as_str() {
        "simplified" => Script::SimplifiedChinese,
        "traditional" => Script::TraditionalChinese,
        "japanese" => Script::Japanese,
        "korean" => Script::Korean,
        _ => Script::Auto,
    }
}

fn parse_format(format: &str) -> OutputFormat {
    match format.to_lowercase().as_str() {
        "detailed" => OutputFormat::Detailed,
        "verbose" => OutputFormat::Verbose,
        _ => OutputFormat::Simple,
    }
}

fn main() {
    let args = Args::parse();
    let normalizer = TextNormalizer::new();

    let result = if args.validate {
        normalizer.validate(&args.text)
    } else {
        let target_script = if args.target == "auto" {
            None
        } else {
            Some(parse_script(&args.target))
        };
        normalizer.normalize(&args.text, target_script)
    };

    match parse_format(&args.format) {
        OutputFormat::Simple => {
            println!("{}", result.normalized);
        }
        OutputFormat::Detailed => {
            println!("Original: {}", result.original);
            println!("Normalized: {}", result.normalized);
            println!("Detected Script: {:?}", result.detected_script);
            println!("Processing Time: {}ms", result.processing_time_ms);
        }
        OutputFormat::Verbose => {
            println!("Original: {}", result.original);
            println!("Normalized: {}", result.normalized);
            println!("Detected Script: {:?}", result.detected_script);
            println!("Processing Time: {}ms", result.processing_time_ms);
            println!();
            println!("Changes:");
            for change in &result.changes {
                println!(
                    "  Position {}: {} → {} ({:?})",
                    change.position,
                    change.original_char,
                    change.normalized_char,
                    change.change_type
                );
                if args.verbose {
                    println!("    Reason: {}", change.reason);
                }
            }
        }
    }
}
