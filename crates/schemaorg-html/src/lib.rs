pub mod microdata;
pub mod extractor;
pub mod converter;

pub use microdata::*;
pub use extractor::*;
pub use converter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microdata_extraction() {
        let html = r#"
            <div itemscope itemtype="https://schema.org/Person">
                <span itemprop="name">Jane Doe</span>
                <a itemprop="url" href="https://example.com">Website</a>
                <div itemprop="worksFor" itemscope itemtype="https://schema.org/Organization">
                    <span itemprop="name">Antinna</span>
                </div>
            </div>
        "#;

        let entities = HtmlExtractor::extract_all(html);
        assert_eq!(entities.len(), 1);

        let person = &entities[0];
        assert_eq!(person.get_types(), vec!["Person"]);
        assert_eq!(person.properties.get("name").unwrap().as_str().unwrap(), "Jane Doe");
        assert_eq!(person.properties.get("url").unwrap().as_str().unwrap(), "https://example.com");

        let works_for = person.properties.get("worksFor").unwrap().as_object().unwrap();
        assert_eq!(works_for.get("@type").unwrap().as_str().unwrap(), "Organization");
        assert_eq!(works_for.get("name").unwrap().as_str().unwrap(), "Antinna");
    }

    #[test]
    fn test_embedded_jsonld_extraction() {
        let html = r#"
            <html>
            <head>
                <script type="application/ld+json">
                {
                    "@context": "https://schema.org",
                    "@type": "Product",
                    "name": "Awesome Gadget"
                }
                </script>
            </head>
            </html>
        "#;

        let entities = HtmlExtractor::extract_all(html);
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].get_types(), vec!["Product"]);
        assert_eq!(entities[0].properties.get("name").unwrap().as_str().unwrap(), "Awesome Gadget");
    }

    #[test]
    fn test_bidirectional_conversions() {
        let html_input = r#"
            <div itemscope itemtype="https://schema.org/Person">
                <span itemprop="name">Alice Smith</span>
            </div>
        "#;

        let jsonld_report = FormatConverter::microdata_to_jsonld(html_input);
        assert!(jsonld_report.output.contains("Alice Smith"));
        assert!(jsonld_report.output.contains("Person"));

        let microdata_report = FormatConverter::jsonld_to_microdata(&jsonld_report.output);
        assert!(microdata_report.output.contains("itemscope"));
        assert!(microdata_report.output.contains("itemtype=\"https://schema.org/Person\""));
        assert!(microdata_report.output.contains("itemprop=\"name\""));
    }
}
