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
pub struct PersistenceCronStrategyDto {
    #[serde(rename = "cronExpression", skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PersistenceCronStrategyDto {
    pub fn new() -> PersistenceCronStrategyDto {
        PersistenceCronStrategyDto {
            cron_expression: None,
            name: None,
        }
    }
}
