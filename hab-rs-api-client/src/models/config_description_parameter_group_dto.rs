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
pub struct ConfigDescriptionParameterGroupDto {
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ConfigDescriptionParameterGroupDto {
    pub fn new() -> ConfigDescriptionParameterGroupDto {
        ConfigDescriptionParameterGroupDto {
            advanced: None,
            context: None,
            description: None,
            label: None,
            name: None,
        }
    }
}
