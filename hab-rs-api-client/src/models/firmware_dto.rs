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
pub struct FirmwareDto {
    #[serde(rename = "changelog", skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "modelRestricted", skip_serializing_if = "Option::is_none")]
    pub model_restricted: Option<bool>,
    #[serde(rename = "prerequisiteVersion", skip_serializing_if = "Option::is_none")]
    pub prerequisite_version: Option<String>,
    #[serde(rename = "thingTypeUID", skip_serializing_if = "Option::is_none")]
    pub thing_type_uid: Option<String>,
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl FirmwareDto {
    pub fn new() -> FirmwareDto {
        FirmwareDto {
            changelog: None,
            description: None,
            model: None,
            model_restricted: None,
            prerequisite_version: None,
            thing_type_uid: None,
            vendor: None,
            version: None,
        }
    }
}

