pub mod parser;
pub mod generator;

use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Schema.org Code Generator running...");
    let json_content = fs::read_to_string("fixtures/schemaorg-current-https.jsonld")?;
    let vocab = parser::parse_vocabulary(&json_content)?;

    println!("Parsed {} classes and {} properties", vocab.classes.len(), vocab.properties.len());

    let vocab_code = generator::generate_vocabulary_code(&vocab);
    fs::create_dir_all("crates/schemaorg-vocabulary/src")?;
    fs::write("crates/schemaorg-vocabulary/src/generated_vocabulary.rs", vocab_code)?;

    let types_code = generator::generate_types_code(&vocab);
    fs::write("crates/schemaorg-vocabulary/src/generated_types.rs", types_code)?;

    println!("Generated code written to crates/schemaorg-vocabulary/src/");
    Ok(())
}
