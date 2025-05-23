/*
 * openHAB REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 8
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconSet {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "formats", skip_serializing_if = "Option::is_none")]
    pub formats: Option<std::collections::HashSet<Formats>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl IconSet {
    pub fn new() -> IconSet {
        IconSet {
            description: None,
            formats: None,
            id: None,
            label: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Formats {
    #[serde(rename = "PNG")]
    Png,
    #[serde(rename = "SVG")]
    Svg,
}

impl Default for Formats {
    fn default() -> Formats {
        Self::Png
    }
}
