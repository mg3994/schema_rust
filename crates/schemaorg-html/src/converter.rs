use schemaorg_core::{DynamicEntity, JsonLdParser, JsonLdSerializer, SerializeOptions};
use crate::microdata::MicrodataParser;

pub struct ConversionReport {
    pub output: String,
    pub warnings: Vec<String>,
}

pub struct FormatConverter;

impl FormatConverter {
    pub fn microdata_to_jsonld(html_str: &str) -> ConversionReport {
        let mut warnings = Vec::new();
        let entities = MicrodataParser::parse_html(html_str);

        if entities.is_empty() {
            warnings.push("No Microdata itemscope entities were found in the HTML input.".to_string());
            return ConversionReport {
                output: "[]".to_string(),
                warnings,
            };
        }

        let options = SerializeOptions {
            pretty: true,
            default_context: Some("https://schema.org".to_string()),
        };

        let jsonld = JsonLdSerializer::serialize_graph(&entities, &options)
            .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e));

        ConversionReport {
            output: jsonld,
            warnings,
        }
    }

    pub fn jsonld_to_microdata(json_str: &str) -> ConversionReport {
        let mut warnings = Vec::new();
        let entities = match JsonLdParser::parse_str(json_str) {
            Ok(e) => e,
            Err(err) => {
                return ConversionReport {
                    output: String::new(),
                    warnings: vec![format!("JSON-LD parse error: {}", err)],
                };
            }
        };

        let mut html_out = String::new();

        for entity in entities {
            html_out.push_str(&Self::entity_to_microdata(&entity, 0, &mut warnings));
        }

        ConversionReport {
            output: html_out,
            warnings,
        }
    }

    fn entity_to_microdata(
        entity: &DynamicEntity,
        indent_level: usize,
        warnings: &mut Vec<String>,
    ) -> String {
        let indent = "  ".repeat(indent_level);
        let mut out = String::new();

        let type_str = entity
            .get_types()
            .first()
            .cloned()
            .unwrap_or_else(|| "Thing".to_string());

        let full_itemtype = if type_str.starts_with("http") {
            type_str
        } else {
            format!("https://schema.org/{}", type_str)
        };

        out.push_str(&format!("{}<div itemscope itemtype=\"{}\">\n", indent, full_itemtype));

        for (prop_name, prop_val) in &entity.properties {
            if prop_name.starts_with('@') {
                continue;
            }

            let prop_indent = "  ".repeat(indent_level + 1);

            if let Some(s) = prop_val.as_str() {
                if s.starts_with("http://") || s.starts_with("https://") {
                    out.push_str(&format!("{}<a itemprop=\"{}\" href=\"{}\">{}</a>\n", prop_indent, prop_name, s, s));
                } else {
                    out.push_str(&format!("{}<span itemprop=\"{}\">{}</span>\n", prop_indent, prop_name, s));
                }
            } else if prop_val.is_object() {
                if let Ok(nested_entity) = serde_json::from_value::<DynamicEntity>(prop_val.clone()) {
                    out.push_str(&format!("{}<div itemprop=\"{}\">\n", prop_indent, prop_name));
                    out.push_str(&Self::entity_to_microdata(&nested_entity, indent_level + 2, warnings));
                    out.push_str(&format!("{}</div>\n", prop_indent));
                }
            } else {
                warnings.push(format!("Complex JSON-LD property '{}' value could not be losslessly converted to Microdata string.", prop_name));
            }
        }

        out.push_str(&format!("{}</div>\n", indent));
        out
    }
}
