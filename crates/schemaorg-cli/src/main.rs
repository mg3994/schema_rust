use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use schemaorg_core::{
    HydrationOptions, Hydrator, JsonLdParser, JsonLdSerializer, SerializeOptions, Validator,
};
use schemaorg_html::{FormatConverter, HtmlExtractor};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "schemaorg")]
#[command(about = "Schema.org CLI for validation, hydration, format conversion, and HTML extraction", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a JSON-LD document against Schema.org vocabulary
    Validate {
        /// Path to input JSON-LD file
        file: PathBuf,
    },
    /// Hydrate missing @context or nested @type on a JSON-LD document
    Hydrate {
        /// Path to input JSON-LD file
        file: PathBuf,
        /// Optional root type hint (e.g. Person, Product)
        #[arg(short, long)]
        root_type: Option<String>,
    },
    /// Extract Microdata and JSON-LD from an HTML file
    Extract {
        /// Path to input HTML file
        file: PathBuf,
    },
    /// Convert Microdata HTML to JSON-LD
    Convert {
        /// Path to input Microdata HTML file
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { file } => {
            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file: {:?}", file))?;
            let entities = JsonLdParser::parse_str(&content)?;
            println!("Parsed {} top-level entity(ies).", entities.len());

            for (idx, entity) in entities.iter().enumerate() {
                println!("\n--- Entity [{}] ---", idx + 1);
                let report = Validator::validate(entity);
                println!("Is Valid: {}", report.is_valid);
                for diag in report.diagnostics {
                    println!("  [{:?}] Path: {} | Code: {} | {}", diag.level, diag.path, diag.code, diag.message);
                }
            }
        }
        Commands::Hydrate { file, root_type } => {
            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file: {:?}", file))?;
            let entities = JsonLdParser::parse_str(&content)?;
            if entities.is_empty() {
                println!("No entities found in JSON-LD document.");
                return Ok(());
            }

            let options = HydrationOptions {
                root_type,
                add_default_context: true,
                infer_nested_types: true,
            };

            let (hydrated, report) = Hydrator::hydrate(&entities[0], &options);
            println!("Inferred types: {:?}", report.inferred_types);
            println!("Ambiguities: {:?}", report.ambiguities);

            let output = JsonLdSerializer::serialize(&hydrated, &SerializeOptions::default())?;
            println!("\n=== Hydrated Document ===\n{}", output);
        }
        Commands::Extract { file } => {
            let html_content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read HTML file: {:?}", file))?;
            let entities = HtmlExtractor::extract_all(&html_content);
            println!("Extracted {} entity(ies) from HTML:", entities.len());

            let output = JsonLdSerializer::serialize_graph(&entities, &SerializeOptions::default())?;
            println!("{}", output);
        }
        Commands::Convert { file } => {
            let html_content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file: {:?}", file))?;
            let report = FormatConverter::microdata_to_jsonld(&html_content);
            if !report.warnings.is_empty() {
                println!("Conversion Warnings: {:?}", report.warnings);
            }
            println!("{}", report.output);
        }
    }

    Ok(())
}
