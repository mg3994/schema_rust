use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaGraphNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub node_type: Option<serde_json::Value>,
    #[serde(rename = "rdfs:label")]
    pub label: Option<serde_json::Value>,
    #[serde(rename = "rdfs:comment")]
    pub comment: Option<serde_json::Value>,
    #[serde(rename = "rdfs:subClassOf")]
    pub sub_class_of: Option<serde_json::Value>,
    #[serde(rename = "schema:domainIncludes")]
    pub domain_includes: Option<serde_json::Value>,
    #[serde(rename = "schema:rangeIncludes")]
    pub range_includes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaGraph {
    #[serde(rename = "@graph")]
    pub graph: Vec<SchemaGraphNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedClass {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub parents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedProperty {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub domains: Vec<String>,
    pub ranges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedVocabulary {
    pub classes: HashMap<String, ParsedClass>,
    pub properties: HashMap<String, ParsedProperty>,
}

fn extract_id_list(value: &Option<serde_json::Value>) -> Vec<String> {
    let mut results = Vec::new();
    if let Some(val) = value {
        if let Some(arr) = val.as_array() {
            for item in arr {
                if let Some(obj) = item.as_object() {
                    if let Some(id) = obj.get("@id").and_then(|v| v.as_str()) {
                        results.push(clean_schema_id(id));
                    }
                }
            }
        } else if let Some(obj) = val.as_object() {
            if let Some(id) = obj.get("@id").and_then(|v| v.as_str()) {
                results.push(clean_schema_id(id));
            }
        }
    }
    results
}

fn extract_string_label(value: &Option<serde_json::Value>) -> String {
    if let Some(val) = value {
        if let Some(s) = val.as_str() {
            return s.to_string();
        } else if let Some(obj) = val.as_object() {
            if let Some(s) = obj.get("@value").and_then(|v| v.as_str()) {
                return s.to_string();
            }
        }
    }
    String::new()
}

fn clean_schema_id(id: &str) -> String {
    id.trim_start_matches("schema:")
        .trim_start_matches("https://schema.org/")
        .trim_start_matches("http://schema.org/")
        .to_string()
}

pub fn parse_vocabulary(json_str: &str) -> anyhow::Result<ParsedVocabulary> {
    let graph: SchemaGraph = serde_json::from_str(json_str)?;
    let mut classes = HashMap::new();
    let mut properties = HashMap::new();

    for node in graph.graph {
        let clean_id = clean_schema_id(&node.id);
        let name = extract_string_label(&node.label);
        let comment = extract_string_label(&node.comment);

        let is_class = match &node.node_type {
            Some(serde_json::Value::String(s)) => s == "rdfs:Class",
            Some(serde_json::Value::Array(arr)) => {
                arr.iter().any(|v| v.as_str() == Some("rdfs:Class"))
            }
            _ => false,
        };

        let is_property = match &node.node_type {
            Some(serde_json::Value::String(s)) => s == "rdf:Property",
            Some(serde_json::Value::Array(arr)) => {
                arr.iter().any(|v| v.as_str() == Some("rdf:Property"))
            }
            _ => false,
        };

        if is_class {
            let parents = extract_id_list(&node.sub_class_of);
            classes.insert(
                clean_id.clone(),
                ParsedClass {
                    id: clean_id.clone(),
                    name: if name.is_empty() { clean_id.clone() } else { name },
                    comment,
                    parents,
                },
            );
        } else if is_property {
            let domains = extract_id_list(&node.domain_includes);
            let ranges = extract_id_list(&node.range_includes);
            properties.insert(
                clean_id.clone(),
                ParsedProperty {
                    id: clean_id.clone(),
                    name: if name.is_empty() { clean_id.clone() } else { name },
                    comment,
                    domains,
                    ranges,
                },
            );
        }
    }

    Ok(ParsedVocabulary { classes, properties })
}
