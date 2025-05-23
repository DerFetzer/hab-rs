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
pub trait UiApi: Send + Sync {
    /// POST /ui/components/{namespace}
    ///
    ///
    async fn add_ui_component_to_namespace<'namespace, 'root_ui_component>(
        &self,
        namespace: &'namespace str,
        root_ui_component: Option<models::RootUiComponent>,
    ) -> Result<models::RootUiComponent, Error<AddUiComponentToNamespaceError>>;

    /// GET /ui/components/{namespace}
    ///
    ///
    async fn get_registered_ui_components_in_namespace<'namespace, 'summary>(
        &self,
        namespace: &'namespace str,
        summary: Option<bool>,
    ) -> Result<Vec<models::RootUiComponent>, Error<GetRegisteredUiComponentsInNamespaceError>>;

    /// GET /ui/components/{namespace}/{componentUID}
    ///
    ///
    async fn get_ui_component_in_namespace<'namespace, 'component_uid>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
    ) -> Result<models::RootUiComponent, Error<GetUiComponentInNamespaceError>>;

    /// GET /ui/tiles
    ///
    ///
    async fn get_ui_tiles(&self) -> Result<Vec<models::TileDto>, Error<GetUiTilesError>>;

    /// DELETE /ui/components/{namespace}/{componentUID}
    ///
    ///
    async fn remove_ui_component_from_namespace<'namespace, 'component_uid>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
    ) -> Result<(), Error<RemoveUiComponentFromNamespaceError>>;

    /// PUT /ui/components/{namespace}/{componentUID}
    ///
    ///
    async fn update_ui_component_in_namespace<'namespace, 'component_uid, 'root_ui_component>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
        root_ui_component: Option<models::RootUiComponent>,
    ) -> Result<models::RootUiComponent, Error<UpdateUiComponentInNamespaceError>>;
}

pub struct UiApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl UiApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl UiApi for UiApiClient {
    async fn add_ui_component_to_namespace<'namespace, 'root_ui_component>(
        &self,
        namespace: &'namespace str,
        root_ui_component: Option<models::RootUiComponent>,
    ) -> Result<models::RootUiComponent, Error<AddUiComponentToNamespaceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/ui/components/{namespace}",
            local_var_configuration.base_path,
            namespace = crate::apis::urlencode(namespace)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        local_var_req_builder = local_var_req_builder.json(&root_ui_component);

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
                        "Received `text/plain` content type response that cannot be converted to `models::RootUiComponent`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::RootUiComponent`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<AddUiComponentToNamespaceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_registered_ui_components_in_namespace<'namespace, 'summary>(
        &self,
        namespace: &'namespace str,
        summary: Option<bool>,
    ) -> Result<Vec<models::RootUiComponent>, Error<GetRegisteredUiComponentsInNamespaceError>>
    {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/ui/components/{namespace}",
            local_var_configuration.base_path,
            namespace = crate::apis::urlencode(namespace)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = summary {
            local_var_req_builder =
                local_var_req_builder.query(&[("summary", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;models::RootUiComponent&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::RootUiComponent&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetRegisteredUiComponentsInNamespaceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_ui_component_in_namespace<'namespace, 'component_uid>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
    ) -> Result<models::RootUiComponent, Error<GetUiComponentInNamespaceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/ui/components/{namespace}/{componentUID}",
            local_var_configuration.base_path,
            namespace = crate::apis::urlencode(namespace),
            componentUID = crate::apis::urlencode(component_uid)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
                        "Received `text/plain` content type response that cannot be converted to `models::RootUiComponent`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::RootUiComponent`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetUiComponentInNamespaceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_ui_tiles(&self) -> Result<Vec<models::TileDto>, Error<GetUiTilesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/ui/tiles", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;models::TileDto&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::TileDto&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetUiTilesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn remove_ui_component_from_namespace<'namespace, 'component_uid>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
    ) -> Result<(), Error<RemoveUiComponentFromNamespaceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/ui/components/{namespace}/{componentUID}",
            local_var_configuration.base_path,
            namespace = crate::apis::urlencode(namespace),
            componentUID = crate::apis::urlencode(component_uid)
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
            let local_var_entity: Option<RemoveUiComponentFromNamespaceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn update_ui_component_in_namespace<'namespace, 'component_uid, 'root_ui_component>(
        &self,
        namespace: &'namespace str,
        component_uid: &'component_uid str,
        root_ui_component: Option<models::RootUiComponent>,
    ) -> Result<models::RootUiComponent, Error<UpdateUiComponentInNamespaceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/ui/components/{namespace}/{componentUID}",
            local_var_configuration.base_path,
            namespace = crate::apis::urlencode(namespace),
            componentUID = crate::apis::urlencode(component_uid)
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
        local_var_req_builder = local_var_req_builder.json(&root_ui_component);

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
                        "Received `text/plain` content type response that cannot be converted to `models::RootUiComponent`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::RootUiComponent`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<UpdateUiComponentInNamespaceError> =
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

/// struct for typed errors of method [UiApi::add_ui_component_to_namespace]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUiComponentToNamespaceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [UiApi::get_registered_ui_components_in_namespace]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRegisteredUiComponentsInNamespaceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [UiApi::get_ui_component_in_namespace]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUiComponentInNamespaceError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [UiApi::get_ui_tiles]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUiTilesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [UiApi::remove_ui_component_from_namespace]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUiComponentFromNamespaceError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [UiApi::update_ui_component_in_namespace]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUiComponentInNamespaceError {
    Status404(),
    UnknownValue(serde_json::Value),
}
