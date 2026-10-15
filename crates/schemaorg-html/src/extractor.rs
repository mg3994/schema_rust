use crate::microdata::MicrodataParser;
use kuchikiki::traits::*;
use schemaorg_core::{DynamicEntity, JsonLdParser};

pub struct HtmlExtractor;

impl HtmlExtractor {
    pub fn extract_all(html_str: &str) -> Vec<DynamicEntity> {
        let mut results = Vec::new();

        let microdata_entities = MicrodataParser::parse_html(html_str);
        results.extend(microdata_entities);

        let document = kuchikiki::parse_html().one(html_str);
        if let Ok(scripts) = document.select("script[type='application/ld+json']") {
            for script in scripts {
                let text = script.as_node().text_contents();
                if let Ok(json_entities) = JsonLdParser::parse_str(&text) {
                    results.extend(json_entities);
                }
            }
        }

        results
    }
}
