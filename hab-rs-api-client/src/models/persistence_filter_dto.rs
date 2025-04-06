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
pub struct PersistenceFilterDto {
    #[serde(rename = "inverted", skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
    #[serde(rename = "lower", skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "relative", skip_serializing_if = "Option::is_none")]
    pub relative: Option<bool>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "upper", skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl PersistenceFilterDto {
    pub fn new() -> PersistenceFilterDto {
        PersistenceFilterDto {
            inverted: None,
            lower: None,
            name: None,
            relative: None,
            unit: None,
            upper: None,
            value: None,
            values: None,
        }
    }
}
