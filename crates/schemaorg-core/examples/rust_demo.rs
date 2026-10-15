use schemaorg_core::{
    HydrationOptions, Hydrator, JsonLdParser, JsonLdSerializer, SerializeOptions,
    Validator,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== High-Performance Schema.org Rust Engine Demo ===\n");

    let raw_json = r#"{
        "name": "Jane Doe",
        "worksFor": {
            "name": "Antinna"
        }
    }"#;

    println!("1. Parsing raw unhydrated JSON:");
    let entities = JsonLdParser::parse_str(raw_json)?;
    let entity = &entities[0];

    println!("\n2. Hydrating entity (inferring missing @context and nested @type)...");
    let options = HydrationOptions {
        root_type: Some("Person".to_string()),
        add_default_context: true,
        infer_nested_types: true,
    };
    let (hydrated, report) = Hydrator::hydrate(entity, &options);

    println!("Inferred types during hydration:");
    for (path, inferred_type) in &report.inferred_types {
        println!("  Path: {} -> Inferred @type: {}", path, inferred_type);
    }

    println!("\n3. Validating hydrated entity against Schema.org vocabulary...");
    let val_report = Validator::validate(&hydrated);
    println!("Is Valid: {}", val_report.is_valid);

    println!("\n4. Serialized Hydrated JSON-LD:");
    let output_json = JsonLdSerializer::serialize(&hydrated, &SerializeOptions::default())?;
    println!("{}", output_json);

    Ok(())
}
