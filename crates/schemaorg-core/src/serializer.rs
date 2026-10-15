use crate::model::DynamicEntity;
use serde_json::Error;

pub struct SerializeOptions {
    pub pretty: bool,
    pub default_context: Option<String>,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        Self {
            pretty: true,
            default_context: Some("https://schema.org".to_string()),
        }
    }
}

pub struct JsonLdSerializer;

impl JsonLdSerializer {
    pub fn serialize(entity: &DynamicEntity, options: &SerializeOptions) -> Result<String, Error> {
        let mut clone = entity.clone();
        if clone.context.is_none() {
            if let Some(ctx) = &options.default_context {
                clone.context = Some(serde_json::Value::String(ctx.clone()));
            }
        }

        if options.pretty {
            serde_json::to_string_pretty(&clone)
        } else {
            serde_json::to_string(&clone)
        }
    }

    pub fn serialize_graph(entities: &[DynamicEntity], options: &SerializeOptions) -> Result<String, Error> {
        if entities.len() == 1 {
            Self::serialize(&entities[0], options)
        } else {
            let mut wrapper = serde_json::Map::new();
            if let Some(ctx) = &options.default_context {
                wrapper.insert("@context".to_string(), serde_json::Value::String(ctx.clone()));
            }
            let graph_val = serde_json::to_value(entities)?;
            wrapper.insert("@graph".to_string(), graph_val);

            if options.pretty {
                serde_json::to_string_pretty(&wrapper)
            } else {
                serde_json::to_string(&wrapper)
            }
        }
    }
}
