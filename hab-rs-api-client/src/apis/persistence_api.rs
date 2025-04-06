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
pub trait PersistenceApi: Send + Sync {
    /// DELETE /persistence/items/{itemname}
    ///
    ///
    async fn delete_item_from_persistence_service<'service_id, 'itemname, 'starttime, 'endtime>(
        &self,
        service_id: &'service_id str,
        itemname: &'itemname str,
        starttime: &'starttime str,
        endtime: &'endtime str,
    ) -> Result<Vec<String>, Error<DeleteItemFromPersistenceServiceError>>;

    /// DELETE /persistence/{serviceId}
    ///
    ///
    async fn delete_persistence_service_configuration<'service_id>(
        &self,
        service_id: &'service_id str,
    ) -> Result<(), Error<DeletePersistenceServiceConfigurationError>>;

    /// GET /persistence/items/{itemname}
    ///
    ///
    async fn get_item_data_from_persistence_service<
        'itemname,
        'service_id,
        'starttime,
        'endtime,
        'page,
        'pagelength,
        'boundary,
        'item_state,
    >(
        &self,
        itemname: &'itemname str,
        service_id: Option<&'service_id str>,
        starttime: Option<&'starttime str>,
        endtime: Option<&'endtime str>,
        page: Option<i32>,
        pagelength: Option<i32>,
        boundary: Option<bool>,
        item_state: Option<bool>,
    ) -> Result<models::ItemHistoryDto, Error<GetItemDataFromPersistenceServiceError>>;

    /// GET /persistence/items
    ///
    ///
    async fn get_items_for_persistence_service<'service_id>(
        &self,
        service_id: Option<&'service_id str>,
    ) -> Result<Vec<models::PersistenceItemInfo>, Error<GetItemsForPersistenceServiceError>>;

    /// GET /persistence/{serviceId}
    ///
    ///
    async fn get_persistence_service_configuration<'service_id>(
        &self,
        service_id: &'service_id str,
    ) -> Result<
        models::PersistenceServiceConfigurationDto,
        Error<GetPersistenceServiceConfigurationError>,
    >;

    /// GET /persistence
    ///
    ///
    async fn get_persistence_services<'accept_language>(
        &self,
        accept_language: Option<&'accept_language str>,
    ) -> Result<Vec<models::PersistenceServiceDto>, Error<GetPersistenceServicesError>>;

    /// PUT /persistence/{serviceId}
    ///
    ///
    async fn put_persistence_service_configuration<
        'service_id,
        'persistence_service_configuration_dto,
    >(
        &self,
        service_id: &'service_id str,
        persistence_service_configuration_dto: models::PersistenceServiceConfigurationDto,
    ) -> Result<
        models::PersistenceServiceConfigurationDto,
        Error<PutPersistenceServiceConfigurationError>,
    >;

    /// PUT /persistence/items/{itemname}
    ///
    ///
    async fn store_item_data_in_persistence_service<'itemname, 'time, 'state, 'service_id>(
        &self,
        itemname: &'itemname str,
        time: &'time str,
        state: &'state str,
        service_id: Option<&'service_id str>,
    ) -> Result<(), Error<StoreItemDataInPersistenceServiceError>>;
}

pub struct PersistenceApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl PersistenceApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl PersistenceApi for PersistenceApiClient {
    async fn delete_item_from_persistence_service<'service_id, 'itemname, 'starttime, 'endtime>(
        &self,
        service_id: &'service_id str,
        itemname: &'itemname str,
        starttime: &'starttime str,
        endtime: &'endtime str,
    ) -> Result<Vec<String>, Error<DeleteItemFromPersistenceServiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/items/{itemname}",
            local_var_configuration.base_path,
            itemname = crate::apis::urlencode(itemname)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        local_var_req_builder =
            local_var_req_builder.query(&[("serviceId", &service_id.to_string())]);
        local_var_req_builder =
            local_var_req_builder.query(&[("starttime", &starttime.to_string())]);
        local_var_req_builder = local_var_req_builder.query(&[("endtime", &endtime.to_string())]);
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
            let local_var_entity: Option<DeleteItemFromPersistenceServiceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn delete_persistence_service_configuration<'service_id>(
        &self,
        service_id: &'service_id str,
    ) -> Result<(), Error<DeletePersistenceServiceConfigurationError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/{serviceId}",
            local_var_configuration.base_path,
            serviceId = crate::apis::urlencode(service_id)
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
            let local_var_entity: Option<DeletePersistenceServiceConfigurationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_item_data_from_persistence_service<
        'itemname,
        'service_id,
        'starttime,
        'endtime,
        'page,
        'pagelength,
        'boundary,
        'item_state,
    >(
        &self,
        itemname: &'itemname str,
        service_id: Option<&'service_id str>,
        starttime: Option<&'starttime str>,
        endtime: Option<&'endtime str>,
        page: Option<i32>,
        pagelength: Option<i32>,
        boundary: Option<bool>,
        item_state: Option<bool>,
    ) -> Result<models::ItemHistoryDto, Error<GetItemDataFromPersistenceServiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/items/{itemname}",
            local_var_configuration.base_path,
            itemname = crate::apis::urlencode(itemname)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = service_id {
            local_var_req_builder =
                local_var_req_builder.query(&[("serviceId", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = starttime {
            local_var_req_builder =
                local_var_req_builder.query(&[("starttime", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = endtime {
            local_var_req_builder =
                local_var_req_builder.query(&[("endtime", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = pagelength {
            local_var_req_builder =
                local_var_req_builder.query(&[("pagelength", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = boundary {
            local_var_req_builder =
                local_var_req_builder.query(&[("boundary", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = item_state {
            local_var_req_builder =
                local_var_req_builder.query(&[("itemState", &local_var_str.to_string())]);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ItemHistoryDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::ItemHistoryDto`")))),
            }
        } else {
            let local_var_entity: Option<GetItemDataFromPersistenceServiceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_items_for_persistence_service<'service_id>(
        &self,
        service_id: Option<&'service_id str>,
    ) -> Result<Vec<models::PersistenceItemInfo>, Error<GetItemsForPersistenceServiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/persistence/items", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = service_id {
            local_var_req_builder =
                local_var_req_builder.query(&[("serviceId", &local_var_str.to_string())]);
        }
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::PersistenceItemInfo&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::PersistenceItemInfo&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetItemsForPersistenceServiceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_persistence_service_configuration<'service_id>(
        &self,
        service_id: &'service_id str,
    ) -> Result<
        models::PersistenceServiceConfigurationDto,
        Error<GetPersistenceServiceConfigurationError>,
    > {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/{serviceId}",
            local_var_configuration.base_path,
            serviceId = crate::apis::urlencode(service_id)
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PersistenceServiceConfigurationDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::PersistenceServiceConfigurationDto`")))),
            }
        } else {
            let local_var_entity: Option<GetPersistenceServiceConfigurationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_persistence_services<'accept_language>(
        &self,
        accept_language: Option<&'accept_language str>,
    ) -> Result<Vec<models::PersistenceServiceDto>, Error<GetPersistenceServicesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/persistence", local_var_configuration.base_path);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::PersistenceServiceDto&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::PersistenceServiceDto&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetPersistenceServicesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn put_persistence_service_configuration<
        'service_id,
        'persistence_service_configuration_dto,
    >(
        &self,
        service_id: &'service_id str,
        persistence_service_configuration_dto: models::PersistenceServiceConfigurationDto,
    ) -> Result<
        models::PersistenceServiceConfigurationDto,
        Error<PutPersistenceServiceConfigurationError>,
    > {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/{serviceId}",
            local_var_configuration.base_path,
            serviceId = crate::apis::urlencode(service_id)
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
        local_var_req_builder = local_var_req_builder.json(&persistence_service_configuration_dto);

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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PersistenceServiceConfigurationDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::PersistenceServiceConfigurationDto`")))),
            }
        } else {
            let local_var_entity: Option<PutPersistenceServiceConfigurationError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn store_item_data_in_persistence_service<'itemname, 'time, 'state, 'service_id>(
        &self,
        itemname: &'itemname str,
        time: &'time str,
        state: &'state str,
        service_id: Option<&'service_id str>,
    ) -> Result<(), Error<StoreItemDataInPersistenceServiceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/persistence/items/{itemname}",
            local_var_configuration.base_path,
            itemname = crate::apis::urlencode(itemname)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = service_id {
            local_var_req_builder =
                local_var_req_builder.query(&[("serviceId", &local_var_str.to_string())]);
        }
        local_var_req_builder = local_var_req_builder.query(&[("time", &time.to_string())]);
        local_var_req_builder = local_var_req_builder.query(&[("state", &state.to_string())]);
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
            let local_var_entity: Option<StoreItemDataInPersistenceServiceError> =
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

/// struct for typed errors of method [`delete_item_from_persistence_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteItemFromPersistenceServiceError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_persistence_service_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePersistenceServiceConfigurationError {
    Status404(),
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_item_data_from_persistence_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetItemDataFromPersistenceServiceError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_items_for_persistence_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetItemsForPersistenceServiceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_persistence_service_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersistenceServiceConfigurationError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_persistence_services`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPersistenceServicesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_persistence_service_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutPersistenceServiceConfigurationError {
    Status400(),
    Status405(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`store_item_data_in_persistence_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoreItemDataInPersistenceServiceError {
    Status404(),
    UnknownValue(serde_json::Value),
}
