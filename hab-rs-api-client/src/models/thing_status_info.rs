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
pub struct ThingStatusInfo {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "statusDetail", skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<StatusDetail>,
}

impl ThingStatusInfo {
    pub fn new() -> ThingStatusInfo {
        ThingStatusInfo {
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
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "OFFLINE")]
    Offline,
    #[serde(rename = "REMOVING")]
    Removing,
    #[serde(rename = "REMOVED")]
    Removed,
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
    #[serde(rename = "NOT_YET_READY")]
    NotYetReady,
    #[serde(rename = "HANDLER_MISSING_ERROR")]
    HandlerMissingError,
    #[serde(rename = "HANDLER_REGISTERING_ERROR")]
    HandlerRegisteringError,
    #[serde(rename = "HANDLER_INITIALIZING_ERROR")]
    HandlerInitializingError,
    #[serde(rename = "HANDLER_CONFIGURATION_PENDING")]
    HandlerConfigurationPending,
    #[serde(rename = "CONFIGURATION_PENDING")]
    ConfigurationPending,
    #[serde(rename = "COMMUNICATION_ERROR")]
    CommunicationError,
    #[serde(rename = "CONFIGURATION_ERROR")]
    ConfigurationError,
    #[serde(rename = "BRIDGE_OFFLINE")]
    BridgeOffline,
    #[serde(rename = "FIRMWARE_UPDATING")]
    FirmwareUpdating,
    #[serde(rename = "DUTY_CYCLE")]
    DutyCycle,
    #[serde(rename = "BRIDGE_UNINITIALIZED")]
    BridgeUninitialized,
    #[serde(rename = "GONE")]
    Gone,
    #[serde(rename = "DISABLED")]
    Disabled,
}

impl Default for StatusDetail {
    fn default() -> StatusDetail {
        Self::None
    }
}
