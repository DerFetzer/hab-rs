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
pub struct ChannelDefinitionDto {
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "stateDescription", skip_serializing_if = "Option::is_none")]
    pub state_description: Option<Box<models::StateDescription>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "typeUID", skip_serializing_if = "Option::is_none")]
    pub type_uid: Option<String>,
}

impl ChannelDefinitionDto {
    pub fn new() -> ChannelDefinitionDto {
        ChannelDefinitionDto {
            advanced: None,
            category: None,
            description: None,
            id: None,
            label: None,
            properties: None,
            state_description: None,
            tags: None,
            type_uid: None,
        }
    }
}
