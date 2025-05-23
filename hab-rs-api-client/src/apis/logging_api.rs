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
pub trait LoggingApi: Send + Sync {
    /// GET /logging/{loggerName}
    ///
    ///
    async fn get_logger<'logger_name>(
        &self,
        logger_name: &'logger_name str,
    ) -> Result<models::LoggerInfo, Error<GetLoggerError>>;

    /// GET /logging
    ///
    ///
    async fn get_logger1(&self) -> Result<models::LoggerBean, Error<GetLogger1Error>>;

    /// PUT /logging/{loggerName}
    ///
    ///
    async fn put_logger<'logger_name, 'logger_info>(
        &self,
        logger_name: &'logger_name str,
        logger_info: models::LoggerInfo,
    ) -> Result<(), Error<PutLoggerError>>;

    /// DELETE /logging/{loggerName}
    ///
    ///
    async fn remove_logger<'logger_name>(
        &self,
        logger_name: &'logger_name str,
    ) -> Result<(), Error<RemoveLoggerError>>;
}

pub struct LoggingApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl LoggingApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl LoggingApi for LoggingApiClient {
    async fn get_logger<'logger_name>(
        &self,
        logger_name: &'logger_name str,
    ) -> Result<models::LoggerInfo, Error<GetLoggerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/logging/{loggerName}",
            local_var_configuration.base_path,
            loggerName = crate::apis::urlencode(logger_name)
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
                        "Received `text/plain` content type response that cannot be converted to `models::LoggerInfo`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::LoggerInfo`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetLoggerError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_logger1(&self) -> Result<models::LoggerBean, Error<GetLogger1Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/logging", local_var_configuration.base_path);
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
                        "Received `text/plain` content type response that cannot be converted to `models::LoggerBean`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::LoggerBean`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetLogger1Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn put_logger<'logger_name, 'logger_info>(
        &self,
        logger_name: &'logger_name str,
        logger_info: models::LoggerInfo,
    ) -> Result<(), Error<PutLoggerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/logging/{loggerName}",
            local_var_configuration.base_path,
            loggerName = crate::apis::urlencode(logger_name)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
        local_var_req_builder = local_var_req_builder.json(&logger_info);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<PutLoggerError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn remove_logger<'logger_name>(
        &self,
        logger_name: &'logger_name str,
    ) -> Result<(), Error<RemoveLoggerError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/logging/{loggerName}",
            local_var_configuration.base_path,
            loggerName = crate::apis::urlencode(logger_name)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<RemoveLoggerError> =
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

/// struct for typed errors of method [LoggingApi::get_logger]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoggerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [LoggingApi::get_logger1]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogger1Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [LoggingApi::put_logger]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutLoggerError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [LoggingApi::remove_logger]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveLoggerError {
    UnknownValue(serde_json::Value),
}
