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
pub struct EnrichedThingDto {
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "bridgeUID", skip_serializing_if = "Option::is_none")]
    pub bridge_uid: Option<String>,
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<models::EnrichedChannelDto>>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(rename = "firmwareStatus", skip_serializing_if = "Option::is_none")]
    pub firmware_status: Option<Box<models::FirmwareStatusDto>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "statusInfo", skip_serializing_if = "Option::is_none")]
    pub status_info: Option<Box<models::ThingStatusInfo>>,
    #[serde(rename = "thingTypeUID", skip_serializing_if = "Option::is_none")]
    pub thing_type_uid: Option<String>,
}

impl EnrichedThingDto {
    pub fn new() -> EnrichedThingDto {
        EnrichedThingDto {
            uid: None,
            bridge_uid: None,
            channels: None,
            configuration: None,
            editable: None,
            firmware_status: None,
            label: None,
            location: None,
            properties: None,
            status_info: None,
            thing_type_uid: None,
        }
    }
}
