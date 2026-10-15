use kuchikiki::traits::*;
use kuchikiki::NodeRef;
use schemaorg_core::DynamicEntity;
use std::collections::HashMap;

pub struct MicrodataParser;

impl MicrodataParser {
    pub fn parse_html(html_str: &str) -> Vec<DynamicEntity> {
        let document = kuchikiki::parse_html().one(html_str);
        let mut results = Vec::new();

        for node in document.select("[itemscope]").unwrap() {
            let elem = node.as_node();
            if !Self::is_nested_itemscope(elem) {
                if let Some(entity) = Self::extract_itemscope(elem) {
                    results.push(entity);
                }
            }
        }

        results
    }

    fn is_nested_itemscope(node: &NodeRef) -> bool {
        let mut curr = node.parent();
        while let Some(parent) = curr {
            if let Some(element) = parent.as_element() {
                if element.attributes.borrow().contains("itemscope") {
                    return true;
                }
            }
            curr = parent.parent();
        }
        false
    }

    pub fn extract_itemscope(node: &NodeRef) -> Option<DynamicEntity> {
        let element = node.as_element()?;
        let attrs = element.attributes.borrow();

        let mut entity = DynamicEntity::new().with_context("https://schema.org");

        if let Some(itemtype) = attrs.get("itemtype") {
            let clean_type = itemtype
                .trim_start_matches("https://schema.org/")
                .trim_start_matches("http://schema.org/")
                .trim_start_matches("schema:");
            entity.type_name = Some(serde_json::Value::String(clean_type.to_string()));
        }

        if let Some(itemid) = attrs.get("itemid") {
            entity.id = Some(itemid.to_string());
        }

        let mut properties: HashMap<String, serde_json::Value> = HashMap::new();
        Self::collect_itemprops(node, &mut properties);

        entity.properties = properties;
        Some(entity)
    }

    fn collect_itemprops(node: &NodeRef, props: &mut HashMap<String, serde_json::Value>) {
        for child in node.children() {
            if let Some(element) = child.as_element() {
                let attrs = element.attributes.borrow();
                if let Some(itemprop) = attrs.get("itemprop") {
                    let prop_names: Vec<&str> = itemprop.split_whitespace().collect();

                    let value = if attrs.contains("itemscope") {
                        if let Some(nested_entity) = Self::extract_itemscope(&child) {
                            serde_json::to_value(nested_entity).unwrap_or(serde_json::Value::Null)
                        } else {
                            serde_json::Value::Null
                        }
                    } else {
                        Self::extract_element_value(&child)
                    };

                    for name in prop_names {
                        if let Some(existing) = props.get_mut(name) {
                            if let Some(arr) = existing.as_array_mut() {
                                arr.push(value.clone());
                            } else {
                                let old = existing.clone();
                                *existing = serde_json::Value::Array(vec![old, value.clone()]);
                            }
                        } else {
                            props.insert(name.to_string(), value.clone());
                        }
                    }
                } else if !attrs.contains("itemscope") {
                    Self::collect_itemprops(&child, props);
                }
            }
        }
    }

    fn extract_element_value(node: &NodeRef) -> serde_json::Value {
        if let Some(element) = node.as_element() {
            let name = element.name.local.to_lowercase();
            let attrs = element.attributes.borrow();

            if name == "a" || name == "link" {
                if let Some(href) = attrs.get("href") {
                    return serde_json::Value::String(href.to_string());
                }
            } else if name == "img" || name == "src" || name == "iframe" {
                if let Some(src) = attrs.get("src") {
                    return serde_json::Value::String(src.to_string());
                }
            } else if name == "meta" {
                if let Some(content) = attrs.get("content") {
                    return serde_json::Value::String(content.to_string());
                }
            } else if name == "time" {
                if let Some(datetime) = attrs.get("datetime") {
                    return serde_json::Value::String(datetime.to_string());
                }
            }
        }

        let text = node.text_contents();
        serde_json::Value::String(text.trim().to_string())
    }
}
