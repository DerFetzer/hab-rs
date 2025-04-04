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
pub struct ConfigDescriptionParameter {
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filterCriteria", skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<Vec<models::FilterCriteria>>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "limitToOptions", skip_serializing_if = "Option::is_none")]
    pub limit_to_options: Option<bool>,
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "multiple", skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(rename = "multipleLimit", skip_serializing_if = "Option::is_none")]
    pub multiple_limit: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::ParameterOption>>,
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "stepSize", skip_serializing_if = "Option::is_none")]
    pub step_size: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "unitLabel", skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
    #[serde(rename = "verifyable", skip_serializing_if = "Option::is_none")]
    pub verifyable: Option<bool>,
}

impl ConfigDescriptionParameter {
    pub fn new() -> ConfigDescriptionParameter {
        ConfigDescriptionParameter {
            advanced: None,
            context: None,
            default: None,
            description: None,
            filter_criteria: None,
            group_name: None,
            label: None,
            limit_to_options: None,
            maximum: None,
            minimum: None,
            multiple: None,
            multiple_limit: None,
            name: None,
            options: None,
            pattern: None,
            read_only: None,
            required: None,
            step_size: None,
            r#type: None,
            unit: None,
            unit_label: None,
            verifyable: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "DECIMAL")]
    Decimal,
    #[serde(rename = "BOOLEAN")]
    Boolean,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}

