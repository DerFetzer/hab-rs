/*
 * openHAB REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 8
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{Error, configuration};
use crate::apis::ContentType;
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use std::sync::Arc;

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait ActionsApi: Send + Sync {
    /// POST /actions/{thingUID}/{actionUid}
    ///
    ///
    async fn execute_thing_action<'thing_uid, 'action_uid, 'accept_language, 'request_body>(
        &self,
        thing_uid: &'thing_uid str,
        action_uid: &'action_uid str,
        accept_language: Option<&'accept_language str>,
        request_body: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<String, Error<ExecuteThingActionError>>;

    /// GET /actions/{thingUID}
    ///
    ///
    async fn get_available_actions_for_thing<'thing_uid, 'accept_language>(
        &self,
        thing_uid: &'thing_uid str,
        accept_language: Option<&'accept_language str>,
    ) -> Result<Vec<models::ThingActionDto>, Error<GetAvailableActionsForThingError>>;
}

pub struct ActionsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ActionsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl ActionsApi for ActionsApiClient {
    async fn execute_thing_action<'thing_uid, 'action_uid, 'accept_language, 'request_body>(
        &self,
        thing_uid: &'thing_uid str,
        action_uid: &'action_uid str,
        accept_language: Option<&'accept_language str>,
        request_body: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Result<String, Error<ExecuteThingActionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/actions/{thingUID}/{actionUid}",
            local_var_configuration.base_path,
            thingUID = crate::apis::urlencode(thing_uid),
            actionUid = crate::apis::urlencode(action_uid)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder =
                local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.json(&request_body);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to `String`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `String`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<ExecuteThingActionError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_available_actions_for_thing<'thing_uid, 'accept_language>(
        &self,
        thing_uid: &'thing_uid str,
        accept_language: Option<&'accept_language str>,
    ) -> Result<Vec<models::ThingActionDto>, Error<GetAvailableActionsForThingError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/actions/{thingUID}",
            local_var_configuration.base_path,
            thingUID = crate::apis::urlencode(thing_uid)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder =
                local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;models::ThingActionDto&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::ThingActionDto&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetAvailableActionsForThingError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [ActionsApi::execute_thing_action]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteThingActionError {
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [ActionsApi::get_available_actions_for_thing]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvailableActionsForThingError {
    Status404(),
    UnknownValue(serde_json::Value),
}
