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
pub struct HumanLanguageInterpreterDto {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "locales", skip_serializing_if = "Option::is_none")]
    pub locales: Option<Vec<String>>,
}

impl HumanLanguageInterpreterDto {
    pub fn new() -> HumanLanguageInterpreterDto {
        HumanLanguageInterpreterDto {
            id: None,
            label: None,
            locales: None,
        }
    }
}
