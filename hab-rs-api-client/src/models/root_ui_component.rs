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
pub struct RootUiComponent {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
    pub props: Option<Box<models::ConfigDescriptionDto>>,
    #[serde(rename = "slots", skip_serializing_if = "Option::is_none")]
    pub slots: Option<std::collections::HashMap<String, Vec<models::UiComponent>>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl RootUiComponent {
    pub fn new() -> RootUiComponent {
        RootUiComponent {
            component: None,
            config: None,
            props: None,
            slots: None,
            tags: None,
            timestamp: None,
            r#type: None,
            uid: None,
        }
    }
}
