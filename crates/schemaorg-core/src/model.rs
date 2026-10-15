use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicEntity {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,

    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<serde_json::Value>,

    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

impl DynamicEntity {
    pub fn new() -> Self {
        Self {
            context: None,
            type_name: None,
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn with_type(mut self, type_name: &str) -> Self {
        self.type_name = Some(serde_json::Value::String(type_name.to_string()));
        self
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(serde_json::Value::String(context.to_string()));
        self
    }

    pub fn set_property<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), serde_json::Error> {
        let json_val = serde_json::to_value(value)?;
        self.properties.insert(key.to_string(), json_val);
        Ok(())
    }

    pub fn get_types(&self) -> Vec<String> {
        match &self.type_name {
            Some(serde_json::Value::String(s)) => vec![s.clone()],
            Some(serde_json::Value::Array(arr)) => {
                arr.iter().filter_map(|v| v.as_str().map(String::from)).collect()
            }
            _ => vec![],
        }
    }
}

impl Default for DynamicEntity {
    fn default() -> Self {
        Self::new()
    }
}
