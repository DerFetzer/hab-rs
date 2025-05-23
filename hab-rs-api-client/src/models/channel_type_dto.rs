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
pub struct ChannelTypeDto {
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "commandDescription", skip_serializing_if = "Option::is_none")]
    pub command_description: Option<Box<models::CommandDescription>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "itemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "parameterGroups", skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<Vec<models::ConfigDescriptionParameterGroupDto>>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<models::ConfigDescriptionParameterDto>>,
    #[serde(rename = "stateDescription", skip_serializing_if = "Option::is_none")]
    pub state_description: Option<Box<models::StateDescription>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "unitHint", skip_serializing_if = "Option::is_none")]
    pub unit_hint: Option<String>,
}

impl ChannelTypeDto {
    pub fn new() -> ChannelTypeDto {
        ChannelTypeDto {
            uid: None,
            advanced: None,
            category: None,
            command_description: None,
            description: None,
            item_type: None,
            kind: None,
            label: None,
            parameter_groups: None,
            parameters: None,
            state_description: None,
            tags: None,
            unit_hint: None,
        }
    }
}
