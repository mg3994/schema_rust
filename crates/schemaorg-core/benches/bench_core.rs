use criterion::{criterion_group, criterion_main, Criterion};
use schemaorg_core::{
    DynamicEntity, HydrationOptions, Hydrator, JsonLdParser, Validator,
};

fn benchmark_jsonld_parse(c: &mut Criterion) {
    let json_data = r#"{
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "Jane Doe",
        "jobTitle": "Principal Engineer",
        "worksFor": {
            "name": "Antinna",
            "url": "https://example.com"
        }
    }"#;

    c.bench_function("parse_jsonld", |b| {
        b.iter(|| JsonLdParser::parse_str(json_data).unwrap())
    });
}

fn benchmark_hydration(c: &mut Criterion) {
    let entity = DynamicEntity::new()
        .with_type("Person")
        .with_context("https://schema.org");

    let options = HydrationOptions {
        root_type: Some("Person".to_string()),
        add_default_context: true,
        infer_nested_types: true,
    };

    c.bench_function("hydrate_person", |b| {
        b.iter(|| Hydrator::hydrate(&entity, &options))
    });
}

fn benchmark_validation(c: &mut Criterion) {
    let mut entity = DynamicEntity::new()
        .with_type("Person")
        .with_context("https://schema.org");
    entity.set_property("name", "Jane Doe").unwrap();

    c.bench_function("validate_person", |b| {
        b.iter(|| Validator::validate(&entity))
    });
}

criterion_group!(
    benches,
    benchmark_jsonld_parse,
    benchmark_hydration,
    benchmark_validation
);
criterion_main!(benches);
