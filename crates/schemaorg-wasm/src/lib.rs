use schemaorg_core::{
    DynamicEntity, HydrationOptions, Hydrator, JsonLdParser, JsonLdSerializer, SerializeOptions,
    Validator,
};
use schemaorg_html::HtmlExtractor;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen(js_name = parseJsonLd)]
pub fn parse_jsonld(json_str: &str) -> Result<JsValue, JsValue> {
    let entities = JsonLdParser::parse_str(json_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    to_value(&entities).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = hydrateJsonLd)]
pub fn hydrate_jsonld(json_str: &str, root_type: Option<String>) -> Result<JsValue, JsValue> {
    let entities = JsonLdParser::parse_str(json_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    if entities.is_empty() {
        return Err(JsValue::from_str("No entities found in JSON-LD"));
    }

    let options = HydrationOptions {
        root_type,
        add_default_context: true,
        infer_nested_types: true,
    };

    let (hydrated, report) = Hydrator::hydrate(&entities[0], &options);

    let result = serde_json::json!({
        "entity": hydrated,
        "report": report
    });

    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = validateJsonLd)]
pub fn validate_jsonld(json_str: &str) -> Result<JsValue, JsValue> {
    let entities = JsonLdParser::parse_str(json_str)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    if entities.is_empty() {
        return Err(JsValue::from_str("No entities found in JSON-LD"));
    }

    let report = Validator::validate(&entities[0]);
    to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = extractFromHtml)]
pub fn extract_from_html(html_str: &str) -> Result<JsValue, JsValue> {
    let entities = HtmlExtractor::extract_all(html_str);
    to_value(&entities).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = serializeJsonLd)]
pub fn serialize_jsonld(val: JsValue, pretty: Option<bool>) -> Result<String, JsValue> {
    let entity: DynamicEntity = from_value(val).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let options = SerializeOptions {
        pretty: pretty.unwrap_or(true),
        default_context: Some("https://schema.org".to_string()),
    };
    JsonLdSerializer::serialize(&entity, &options).map_err(|e| JsValue::from_str(&e.to_string()))
}
