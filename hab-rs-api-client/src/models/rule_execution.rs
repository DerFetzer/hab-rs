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
pub struct RuleExecution {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<Box<models::Rule>>,
}

impl RuleExecution {
    pub fn new() -> RuleExecution {
        RuleExecution {
            date: None,
            rule: None,
        }
    }
}
