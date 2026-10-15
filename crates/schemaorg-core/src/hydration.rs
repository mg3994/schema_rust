use crate::model::DynamicEntity;
use schemaorg_vocabulary::SchemaVocabulary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationOptions {
    pub root_type: Option<String>,
    pub add_default_context: bool,
    pub infer_nested_types: bool,
}

impl Default for HydrationOptions {
    fn default() -> Self {
        Self {
            root_type: None,
            add_default_context: true,
            infer_nested_types: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HydrationReport {
    pub inferred_types: Vec<(String, String)>,
    pub ambiguities: Vec<(String, Vec<String>)>,
}

pub struct Hydrator;

impl Hydrator {
    pub fn hydrate(entity: &DynamicEntity, options: &HydrationOptions) -> (DynamicEntity, HydrationReport) {
        let mut result = entity.clone();
        let mut report = HydrationReport::default();

        if options.add_default_context && result.context.is_none() {
            result.context = Some(serde_json::Value::String("https://schema.org".to_string()));
        }

        if result.type_name.is_none() {
            if let Some(rt) = &options.root_type {
                result.type_name = Some(serde_json::Value::String(rt.clone()));
                report.inferred_types.push(("$".to_string(), rt.clone()));
            }
        }

        if options.infer_nested_types {
            Self::hydrate_properties(&mut result.properties, "$", &mut report);
        }

        (result, report)
    }

    fn hydrate_properties(
        props: &mut std::collections::HashMap<String, serde_json::Value>,
        current_path: &str,
        report: &mut HydrationReport,
    ) {
        for (prop_name, prop_val) in props.iter_mut() {
            if prop_name.starts_with('@') {
                continue;
            }

            let path = format!("{}.{}", current_path, prop_name);

            if let Some(obj) = prop_val.as_object_mut() {
                let has_type = obj.contains_key("@type");

                if !has_type {
                    if let Some(ranges) = SchemaVocabulary::get_property_ranges(prop_name) {
                        let non_primitive_ranges: Vec<&str> = ranges
                            .iter()
                            .copied()
                            .filter(|&r| {
                                r != "Text"
                                    && r != "URL"
                                    && r != "Number"
                                    && r != "Boolean"
                                    && r != "Date"
                                    && r != "DateTime"
                            })
                            .collect();

                        if non_primitive_ranges.len() == 1 {
                            let inferred = non_primitive_ranges[0].to_string();
                            obj.insert(
                                "@type".to_string(),
                                serde_json::Value::String(inferred.clone()),
                            );
                            report.inferred_types.push((path.clone(), inferred));
                        } else if non_primitive_ranges.len() > 1 {
                            report.ambiguities.push((
                                path.clone(),
                                non_primitive_ranges.iter().map(|s| s.to_string()).collect(),
                            ));
                        }
                    }
                }

                // Recursively hydrate nested object properties
                let mut nested_props = std::collections::HashMap::new();
                for (k, v) in obj.iter_mut() {
                    nested_props.insert(k.clone(), v.clone());
                }
                Self::hydrate_properties(&mut nested_props, &path, report);
                for (k, v) in nested_props {
                    obj.insert(k, v);
                }
            } else if let Some(arr) = prop_val.as_array_mut() {
                for (idx, elem) in arr.iter_mut().enumerate() {
                    if let Some(obj) = elem.as_object_mut() {
                        let elem_path = format!("{}[{}]", path, idx);
                        if !obj.contains_key("@type") {
                            if let Some(ranges) = SchemaVocabulary::get_property_ranges(prop_name) {
                                let non_primitive_ranges: Vec<&str> = ranges
                                    .iter()
                                    .copied()
                                    .filter(|&r| {
                                        r != "Text"
                                            && r != "URL"
                                            && r != "Number"
                                            && r != "Boolean"
                                            && r != "Date"
                                            && r != "DateTime"
                                    })
                                    .collect();

                                if non_primitive_ranges.len() == 1 {
                                    let inferred = non_primitive_ranges[0].to_string();
                                    obj.insert(
                                        "@type".to_string(),
                                        serde_json::Value::String(inferred.clone()),
                                    );
                                    report.inferred_types.push((elem_path.clone(), inferred));
                                } else if non_primitive_ranges.len() > 1 {
                                    report.ambiguities.push((
                                        elem_path.clone(),
                                        non_primitive_ranges
                                            .iter()
                                            .map(|s| s.to_string())
                                            .collect(),
                                    ));
                                }
                            }
                        }

                        // Recursively hydrate nested object properties inside array element
                        let mut nested_props = std::collections::HashMap::new();
                        for (k, v) in obj.iter_mut() {
                            nested_props.insert(k.clone(), v.clone());
                        }
                        Self::hydrate_properties(&mut nested_props, &elem_path, report);
                        for (k, v) in nested_props {
                            obj.insert(k, v);
                        }
                    }
                }
            }
        }
    }
}
