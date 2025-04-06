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
pub struct StrippedThingTypeDto {
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<bool>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "listed", skip_serializing_if = "Option::is_none")]
    pub listed: Option<bool>,
    #[serde(
        rename = "supportedBridgeTypeUIDs",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_bridge_type_uids: Option<Vec<String>>,
}

impl StrippedThingTypeDto {
    pub fn new() -> StrippedThingTypeDto {
        StrippedThingTypeDto {
            uid: None,
            bridge: None,
            category: None,
            description: None,
            label: None,
            listed: None,
            supported_bridge_type_uids: None,
        }
    }
}
