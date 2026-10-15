pub mod model;
pub mod parser;
pub mod serializer;
pub mod validation;
pub mod hydration;

pub use model::*;
pub use parser::*;
pub use serializer::*;
pub use validation::*;
pub use hydration::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_serialize() {
        let json_data = r#"{
            "@context": "https://schema.org",
            "@type": "Person",
            "name": "Jane Doe",
            "worksFor": {
                "name": "Antinna"
            }
        }"#;

        let entities = JsonLdParser::parse_str(json_data).unwrap();
        assert_eq!(entities.len(), 1);
        let person = &entities[0];
        assert_eq!(person.get_types(), vec!["Person"]);

        let serialized = JsonLdSerializer::serialize(person, &SerializeOptions::default()).unwrap();
        assert!(serialized.contains("Jane Doe"));
    }

    #[test]
    fn test_validation() {
        let mut entity = DynamicEntity::new().with_type("Person");
        entity.set_property("name", "Jane Doe").unwrap();

        let report = Validator::validate(&entity);
        assert!(report.is_valid);
        assert!(report.diagnostics.is_empty());

        let invalid_entity = DynamicEntity::new().with_type("UnknownTypeXYZ");
        let report2 = Validator::validate(&invalid_entity);
        assert!(report2.diagnostics.iter().any(|d| d.code == "UNKNOWN_TYPE"));
    }

    #[test]
    fn test_hydration() {
        let json_data = r#"{
            "name": "Jane Doe",
            "worksFor": {
                "name": "Antinna"
            }
        }"#;

        let entities = JsonLdParser::parse_str(json_data).unwrap();
        let entity = &entities[0];

        let options = HydrationOptions {
            root_type: Some("Person".to_string()),
            add_default_context: true,
            infer_nested_types: true,
        };

        let (hydrated, report) = Hydrator::hydrate(entity, &options);

        assert_eq!(hydrated.context, Some(serde_json::Value::String("https://schema.org".to_string())));
        assert_eq!(hydrated.get_types(), vec!["Person"]);

        let works_for = hydrated.properties.get("worksFor").unwrap().as_object().unwrap();
        assert_eq!(works_for.get("@type").unwrap().as_str().unwrap(), "Organization");

        assert!(report.inferred_types.iter().any(|(p, t)| p == "$.worksFor" && t == "Organization"));
    }
}
