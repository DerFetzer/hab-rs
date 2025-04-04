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
pub struct RuleStatusInfo {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "statusDetail", skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<StatusDetail>,
}

impl RuleStatusInfo {
    pub fn new() -> RuleStatusInfo {
        RuleStatusInfo {
            description: None,
            status: None,
            status_detail: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "UNINITIALIZED")]
    Uninitialized,
    #[serde(rename = "INITIALIZING")]
    Initializing,
    #[serde(rename = "IDLE")]
    Idle,
    #[serde(rename = "RUNNING")]
    Running,
}

impl Default for Status {
    fn default() -> Status {
        Self::Uninitialized
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusDetail {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "HANDLER_MISSING_ERROR")]
    HandlerMissingError,
    #[serde(rename = "HANDLER_INITIALIZING_ERROR")]
    HandlerInitializingError,
    #[serde(rename = "CONFIGURATION_ERROR")]
    ConfigurationError,
    #[serde(rename = "TEMPLATE_MISSING_ERROR")]
    TemplateMissingError,
    #[serde(rename = "INVALID_RULE")]
    InvalidRule,
    #[serde(rename = "DISABLED")]
    Disabled,
}

impl Default for StatusDetail {
    fn default() -> StatusDetail {
        Self::None
    }
}

