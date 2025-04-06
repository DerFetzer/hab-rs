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
pub struct ItemHistoryDto {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::HistoryDataBean>>,
    #[serde(rename = "datapoints", skip_serializing_if = "Option::is_none")]
    pub datapoints: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "totalrecords", skip_serializing_if = "Option::is_none")]
    pub totalrecords: Option<String>,
}

impl ItemHistoryDto {
    pub fn new() -> ItemHistoryDto {
        ItemHistoryDto {
            data: None,
            datapoints: None,
            name: None,
            totalrecords: None,
        }
    }
}
