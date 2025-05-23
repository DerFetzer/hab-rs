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
pub struct PersistenceItemConfigurationDto {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<String>>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
    #[serde(rename = "strategies", skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<String>>,
}

impl PersistenceItemConfigurationDto {
    pub fn new() -> PersistenceItemConfigurationDto {
        PersistenceItemConfigurationDto {
            alias: None,
            filters: None,
            items: None,
            strategies: None,
        }
    }
}
