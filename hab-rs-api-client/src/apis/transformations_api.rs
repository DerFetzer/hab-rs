/*
 * openHAB REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 8
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ContentType;
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};
use std::sync::Arc;

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait TransformationsApi: Send + Sync {
    /// DELETE /transformations/{uid}
    ///
    ///
    async fn delete_transformation<'uid>(
        &self,
        uid: &'uid str,
    ) -> Result<(), Error<DeleteTransformationError>>;

    /// GET /transformations/{uid}
    ///
    ///
    async fn get_transformation<'uid>(
        &self,
        uid: &'uid str,
    ) -> Result<models::Transformation, Error<GetTransformationError>>;

    /// GET /transformations/services
    ///
    ///
    async fn get_transformation_services(
        &self,
    ) -> Result<Vec<String>, Error<GetTransformationServicesError>>;

    /// GET /transformations
    ///
    ///
    async fn get_transformations(
        &self,
    ) -> Result<Vec<models::TransformationDto>, Error<GetTransformationsError>>;

    /// PUT /transformations/{uid}
    ///
    ///
    async fn put_transformation<'uid, 'transformation_dto>(
        &self,
        uid: &'uid str,
        transformation_dto: models::TransformationDto,
    ) -> Result<(), Error<PutTransformationError>>;
}

pub struct TransformationsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TransformationsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl TransformationsApi for TransformationsApiClient {
    async fn delete_transformation<'uid>(
        &self,
        uid: &'uid str,
    ) -> Result<(), Error<DeleteTransformationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/transformations/{uid}",
            local_var_configuration.base_path,
            uid = crate::apis::urlencode(uid)
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
            let local_var_entity: Option<DeleteTransformationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_transformation<'uid>(
        &self,
        uid: &'uid str,
    ) -> Result<models::Transformation, Error<GetTransformationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/transformations/{uid}",
            local_var_configuration.base_path,
            uid = crate::apis::urlencode(uid)
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Transformation`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::Transformation`")))),
            }
        } else {
            let local_var_entity: Option<GetTransformationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_transformation_services(
        &self,
    ) -> Result<Vec<String>, Error<GetTransformationServicesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/transformations/services",
            local_var_configuration.base_path
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;String&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;String&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetTransformationServicesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_transformations(
        &self,
    ) -> Result<Vec<models::TransformationDto>, Error<GetTransformationsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/transformations", local_var_configuration.base_path);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::TransformationDto&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::TransformationDto&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetTransformationsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn put_transformation<'uid, 'transformation_dto>(
        &self,
        uid: &'uid str,
        transformation_dto: models::TransformationDto,
    ) -> Result<(), Error<PutTransformationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/transformations/{uid}",
            local_var_configuration.base_path,
            uid = crate::apis::urlencode(uid)
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
        local_var_req_builder = local_var_req_builder.json(&transformation_dto);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<PutTransformationError> =
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

/// struct for typed errors of method [`delete_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTransformationError {
    Status404(),
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransformationError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transformation_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransformationServicesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transformations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransformationsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_transformation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutTransformationError {
    Status400(),
    Status405(),
    UnknownValue(serde_json::Value),
}
