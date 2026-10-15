use crate::model::DynamicEntity;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("JSON syntax error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid document structure")]
    InvalidStructure,
}

pub struct JsonLdParser;

impl JsonLdParser {
    pub fn parse_str(input: &str) -> Result<Vec<DynamicEntity>, ParseError> {
        let val: serde_json::Value = serde_json::from_str(input)?;
        Self::parse_value(&val)
    }

    pub fn parse_value(val: &serde_json::Value) -> Result<Vec<DynamicEntity>, ParseError> {
        if let Some(arr) = val.as_array() {
            let mut entities = Vec::new();
            for item in arr {
                let entity: DynamicEntity = serde_json::from_value(item.clone())?;
                entities.push(entity);
            }
            Ok(entities)
        } else if let Some(obj) = val.as_object() {
            if let Some(graph) = obj.get("@graph").and_then(|g| g.as_array()) {
                let mut entities = Vec::new();
                for item in graph {
                    let entity: DynamicEntity = serde_json::from_value(item.clone())?;
                    entities.push(entity);
                }
                Ok(entities)
            } else {
                let entity: DynamicEntity = serde_json::from_value(val.clone())?;
                Ok(vec![entity])
            }
        } else {
            Err(ParseError::InvalidStructure)
        }
    }
}
