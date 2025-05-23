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
pub struct GroupItemDto {
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<Box<models::GroupFunctionDto>>,
    #[serde(rename = "groupNames", skip_serializing_if = "Option::is_none")]
    pub group_names: Option<Vec<String>>,
    #[serde(rename = "groupType", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GroupItemDto {
    pub fn new() -> GroupItemDto {
        GroupItemDto {
            category: None,
            function: None,
            group_names: None,
            group_type: None,
            label: None,
            name: None,
            tags: None,
            r#type: None,
        }
    }
}
