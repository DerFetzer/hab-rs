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
pub trait ChannelTypesApi: Send + Sync {
    /// GET /channel-types/{channelTypeUID}
    ///
    ///
    async fn get_channel_type_by_uid<'channel_type_uid, 'accept_language>(
        &self,
        channel_type_uid: &'channel_type_uid str,
        accept_language: Option<&'accept_language str>,
    ) -> Result<models::ChannelTypeDto, Error<GetChannelTypeByUidError>>;

    /// GET /channel-types
    ///
    ///
    async fn get_channel_types<'accept_language, 'prefixes>(
        &self,
        accept_language: Option<&'accept_language str>,
        prefixes: Option<&'prefixes str>,
    ) -> Result<Vec<models::ChannelTypeDto>, Error<GetChannelTypesError>>;

    /// GET /channel-types/{channelTypeUID}/linkableItemTypes
    ///
    ///
    async fn get_linkable_item_types_by_channel_type_uid<'channel_type_uid>(
        &self,
        channel_type_uid: &'channel_type_uid str,
    ) -> Result<Vec<String>, Error<GetLinkableItemTypesByChannelTypeUidError>>;
}

pub struct ChannelTypesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ChannelTypesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl ChannelTypesApi for ChannelTypesApiClient {
    async fn get_channel_type_by_uid<'channel_type_uid, 'accept_language>(
        &self,
        channel_type_uid: &'channel_type_uid str,
        accept_language: Option<&'accept_language str>,
    ) -> Result<models::ChannelTypeDto, Error<GetChannelTypeByUidError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/channel-types/{channelTypeUID}",
            local_var_configuration.base_path,
            channelTypeUID = crate::apis::urlencode(channel_type_uid)
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
        if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
            local_var_req_builder = local_var_req_builder.basic_auth(
                local_var_auth_conf.0.to_owned(),
                local_var_auth_conf.1.to_owned(),
            );
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

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
                        "Received `text/plain` content type response that cannot be converted to `models::ChannelTypeDto`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::ChannelTypeDto`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetChannelTypeByUidError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_channel_types<'accept_language, 'prefixes>(
        &self,
        accept_language: Option<&'accept_language str>,
        prefixes: Option<&'prefixes str>,
    ) -> Result<Vec<models::ChannelTypeDto>, Error<GetChannelTypesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/channel-types", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = prefixes {
            local_var_req_builder =
                local_var_req_builder.query(&[("prefixes", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder =
                local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
        }
        if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
            local_var_req_builder = local_var_req_builder.basic_auth(
                local_var_auth_conf.0.to_owned(),
                local_var_auth_conf.1.to_owned(),
            );
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

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
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;models::ChannelTypeDto&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::ChannelTypeDto&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetChannelTypesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_linkable_item_types_by_channel_type_uid<'channel_type_uid>(
        &self,
        channel_type_uid: &'channel_type_uid str,
    ) -> Result<Vec<String>, Error<GetLinkableItemTypesByChannelTypeUidError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/channel-types/{channelTypeUID}/linkableItemTypes",
            local_var_configuration.base_path,
            channelTypeUID = crate::apis::urlencode(channel_type_uid)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
            local_var_req_builder = local_var_req_builder.basic_auth(
                local_var_auth_conf.0.to_owned(),
                local_var_auth_conf.1.to_owned(),
            );
        };
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

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
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;String&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;String&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetLinkableItemTypesByChannelTypeUidError> =
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

/// struct for typed errors of method [`get_channel_type_by_uid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelTypeByUidError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_channel_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChannelTypesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_linkable_item_types_by_channel_type_uid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLinkableItemTypesByChannelTypeUidError {
    Status404(),
    UnknownValue(serde_json::Value),
}
