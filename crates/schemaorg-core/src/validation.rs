use crate::model::DynamicEntity;
use schemaorg_vocabulary::SchemaVocabulary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub path: String,
    pub message: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct Validator;

impl Validator {
    pub fn validate(entity: &DynamicEntity) -> ValidationReport {
        let mut diagnostics = Vec::new();

        let types = entity.get_types();
        if types.is_empty() {
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                path: "$".to_string(),
                message: "Missing @type property on entity".to_string(),
                code: "MISSING_TYPE".to_string(),
            });
        } else {
            for t in &types {
                if !SchemaVocabulary::is_known_type(t) {
                    diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        path: "$.@type".to_string(),
                        message: format!("Unknown or extension Schema.org type: '{}'", t),
                        code: "UNKNOWN_TYPE".to_string(),
                    });
                }
            }
        }

        // Validate properties against ranges
        for (prop_name, prop_val) in &entity.properties {
            let path = format!("$.{}", prop_name);
            if prop_name.starts_with('@') {
                continue;
            }

            if let Some(expected_ranges) = SchemaVocabulary::get_property_ranges(prop_name) {
                if prop_val.is_object() {
                    if let Ok(nested_entity) = serde_json::from_value::<DynamicEntity>(prop_val.clone()) {
                        let nested_types = nested_entity.get_types();
                        if !nested_types.is_empty() {
                            let mut matches = false;
                            for nt in &nested_types {
                                if expected_ranges.iter().any(|r| r == nt) {
                                    matches = true;
                                    break;
                                }
                            }
                            if !matches && !expected_ranges.is_empty() {
                                diagnostics.push(Diagnostic {
                                    level: DiagnosticLevel::Info,
                                    path: path.clone(),
                                    message: format!(
                                        "Property '{}' has nested type '{:?}', but official rangeIncludes lists: {:?}",
                                        prop_name, nested_types, expected_ranges
                                    ),
                                    code: "RANGE_MISMATCH".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        let is_valid = !diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error);

        ValidationReport {
            is_valid,
            diagnostics,
        }
    }
}
