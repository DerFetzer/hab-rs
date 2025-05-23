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
pub struct Rule {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::Action>>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::Condition>>,
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::Configuration>>,
    #[serde(
        rename = "configurationDescriptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_descriptions: Option<Vec<models::ConfigDescriptionParameter>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modules", skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<models::Module>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "templateUID", skip_serializing_if = "Option::is_none")]
    pub template_uid: Option<String>,
    #[serde(rename = "triggers", skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<models::Trigger>>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            actions: None,
            conditions: None,
            configuration: None,
            configuration_descriptions: None,
            description: None,
            modules: None,
            name: None,
            tags: None,
            template_uid: None,
            triggers: None,
            uid: None,
            visibility: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "VISIBLE")]
    Visible,
    #[serde(rename = "HIDDEN")]
    Hidden,
    #[serde(rename = "EXPERT")]
    Expert,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Self::Visible
    }
}
