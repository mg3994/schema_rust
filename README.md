# SchemaOrg Engine

A high-performance, strongly-typed Schema.org engine built in Rust with WebAssembly bindings and Google Genkit integration.

## Features

- **Schema.org Native Vocabulary**: Generated directly from the official Schema.org JSON-LD release snapshot (`schemaorg-current-https.jsonld`).
- **Dynamic JSON-LD Model**: `DynamicEntity` represents any Schema.org or extension entity dynamically without losing unknown properties or `@context`.
- **Conservative Hydration Engine**: Automatically infers missing `@type` on nested objects based on official property `rangeIncludes` metadata, with diagnostic reporting for ambiguous ranges.
- **Validation Engine**: Performs syntax, structural, and vocabulary checking, returning structured diagnostic reports.
- **HTML Microdata Extraction**: Extracts Microdata and embedded `<script type="application/ld+json">` entities from HTML documents.
- **WebAssembly (WASM)**: High-speed bindings for browser and Node.js environments (`parseJsonLd`, `hydrateJsonLd`, `validateJsonLd`, `extractFromHtml`).
- **Google Genkit Integration**: `@antinna/genkit-schemaorg` TypeScript package providing Zod schema definitions, hydration, and validation for Genkit structured output workflows.

## Workspace Structure

```
.
├── crates/
│   ├── schemaorg-core/        # Semantic model, parser, serializer, validator, hydrator
│   ├── schemaorg-vocabulary/  # Term lookups, ranges, inheritance, generated metadata
│   ├── schemaorg-codegen/     # Codegen binary parsing official JSON-LD definitions
│   ├── schemaorg-html/        # HTML Microdata and script tag extractor
│   └── schemaorg-wasm/        # WebAssembly bindings (wasm-bindgen)
├── integrations/
│   └── genkit/                # @antinna/genkit-schemaorg TypeScript package
├── fixtures/                  # Schema.org current jsonld snapshot
└── examples/                  # Rust examples
```

## Quick Start (Rust)

```rust
use schemaorg_core::{DynamicEntity, Hydrator, HydrationOptions, JsonLdParser, Validator};

fn main() {
    let json_data = r#"{
        "name": "Jane Doe",
        "worksFor": {
            "name": "Antinna"
        }
    }"#;

    // Parse JSON-LD
    let entities = JsonLdParser::parse_str(json_data).unwrap();

    // Hydrate missing root @type and nested object @type
    let options = HydrationOptions {
        root_type: Some("Person".to_string()),
        add_default_context: true,
        infer_nested_types: true,
    };
    let (hydrated, report) = Hydrator::hydrate(&entities[0], &options);

    // Validate against official vocabulary
    let validation = Validator::validate(&hydrated);
    println!("Is Valid: {}", validation.is_valid);
}
```

## Quick Start (Genkit TypeScript Integration)

```typescript
import { genkit } from "genkit";
import { schemaOrgOutput } from "@antinna/genkit-schemaorg";

const ai = genkit({});
const productOutput = schemaOrgOutput("Product");

const response = await ai.generate({
  prompt: "Generate a sample tech product.",
  output: {
    schema: productOutput.schema,
  },
});

const raw = response.output;
if (raw) {
  const hydrated = await productOutput.hydrate(raw);
  const validation = await productOutput.validate(hydrated);
  console.log("Valid:", validation.isValid);
}
```

## Running Codegen & Tests

```bash
# Re-generate vocabulary code from snapshot
cargo run -p schemaorg-codegen

# Run all tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace
```

## License

MIT OR Apache-2.0
