// Auto-generated typed models by schemaorg-codegen.
use serde::{Deserialize, Serialize};

/// A person (alive, dead, undead, or fictional).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Person {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Person {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Person".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// Any offered product or service. For example: a pair of shoes; a concert ticket; the rental of a car; a haircut; or an episode of a TV show streamed online.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Product {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Product {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Product".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// An organization such as a school, NGO, corporation, club, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Organization {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Organization {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Organization".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// An article, such as a news article or piece of investigative report. Newspapers and magazines have articles of many different types and this is intended to cover them all.\n\nSee also [blog post](https://blog.schema.org/2014/09/02/schema-org-support-for-bibliographic-relationships-and-periodicals/).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Article {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Article {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Article".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// An event happening at a certain time and location, such as a concert, lecture, or festival. Ticketing information may be added via the [[offers]] property. Repeated events may be structured as separate Event objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Event {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Event {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Event".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// An offer to transfer some rights to an item or to provide a service — for example, an offer to sell tickets to an event, to rent the DVD of a movie, to stream a TV show over the internet, to repair a motorcycle, or to loan a book.\n\nNote: As the [[businessFunction]] property, which identifies the form of offer (e.g. sell, lease, repair, dispose), defaults to http://purl.org/goodrelations/v1#Sell; an Offer without a defined businessFunction value can be assumed to be an offer to sell.\n\nFor [GTIN](http://www.gs1.org/barcodes/technical/idkeys/gtin)-related fields, see [Check Digit calculator](http://www.gs1.org/barcodes/support/check_digit_calculator) and [validation guide](http://www.gs1us.org/resources/standards/gtin-validation-guide) from [GS1](http://www.gs1.org/).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Offer {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Offer {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Offer".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// Entities that have a somewhat fixed, physical extension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Place {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Place {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Place".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// The mailing address.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PostalAddress {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PostalAddress {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("PostalAddress".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// The most generic kind of creative work, including books, movies, photographs, software programs, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreativeWork {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreativeWork {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("CreativeWork".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}

/// The most generic type of item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Thing {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            context: Some("https://schema.org".to_string()),
            type_name: Some("Thing".to_string()),
            id: None,
            name: None,
            description: None,
            url: None,
        }
    }
}
