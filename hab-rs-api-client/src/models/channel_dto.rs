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
pub struct ChannelDto {
    #[serde(rename = "autoUpdatePolicy", skip_serializing_if = "Option::is_none")]
    pub auto_update_policy: Option<String>,
    #[serde(rename = "channelTypeUID", skip_serializing_if = "Option::is_none")]
    pub channel_type_uid: Option<String>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "defaultTags", skip_serializing_if = "Option::is_none")]
    pub default_tags: Option<Vec<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "itemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl ChannelDto {
    pub fn new() -> ChannelDto {
        ChannelDto {
            auto_update_policy: None,
            channel_type_uid: None,
            configuration: None,
            default_tags: None,
            description: None,
            id: None,
            item_type: None,
            kind: None,
            label: None,
            properties: None,
            uid: None,
        }
    }
}

