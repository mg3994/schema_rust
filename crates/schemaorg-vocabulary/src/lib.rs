pub mod metadata;
pub mod generated_vocabulary;

pub use metadata::*;
pub use generated_vocabulary::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_lookups() {
        assert!(SchemaVocabulary::is_known_type("Person"));
        assert!(SchemaVocabulary::is_known_type("https://schema.org/Product"));
        assert!(SchemaVocabulary::is_known_type("schema:Organization"));
        assert!(!SchemaVocabulary::is_known_type("NonExistentType123"));

        let works_for_ranges = SchemaVocabulary::get_property_ranges("worksFor").unwrap();
        assert!(works_for_ranges.contains(&"Organization"));

        assert_eq!(
            SchemaVocabulary::get_unambiguous_range_type("worksFor"),
            Some("Organization")
        );
    }
}
