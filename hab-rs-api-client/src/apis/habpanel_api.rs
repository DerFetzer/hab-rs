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
pub trait HabpanelApi: Send + Sync {
    /// GET /habpanel/gallery/{galleryName}/widgets
    ///
    ///
    async fn get_gallery_widget_list<'gallery_name>(
        &self,
        gallery_name: &'gallery_name str,
    ) -> Result<Vec<models::GalleryWidgetsListItem>, Error<GetGalleryWidgetListError>>;

    /// GET /habpanel/gallery/{galleryName}/widgets/{id}
    ///
    ///
    async fn get_gallery_widgets_item<'gallery_name, 'id>(
        &self,
        gallery_name: &'gallery_name str,
        id: &'id str,
    ) -> Result<models::GalleryItem, Error<GetGalleryWidgetsItemError>>;
}

pub struct HabpanelApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl HabpanelApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl HabpanelApi for HabpanelApiClient {
    async fn get_gallery_widget_list<'gallery_name>(
        &self,
        gallery_name: &'gallery_name str,
    ) -> Result<Vec<models::GalleryWidgetsListItem>, Error<GetGalleryWidgetListError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/habpanel/gallery/{galleryName}/widgets",
            local_var_configuration.base_path,
            galleryName = crate::apis::urlencode(gallery_name)
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
                        "Received `text/plain` content type response that cannot be converted to `Vec&lt;models::GalleryWidgetsListItem&gt;`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::GalleryWidgetsListItem&gt;`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetGalleryWidgetListError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_gallery_widgets_item<'gallery_name, 'id>(
        &self,
        gallery_name: &'gallery_name str,
        id: &'id str,
    ) -> Result<models::GalleryItem, Error<GetGalleryWidgetsItemError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/habpanel/gallery/{galleryName}/widgets/{id}",
            local_var_configuration.base_path,
            galleryName = crate::apis::urlencode(gallery_name),
            id = crate::apis::urlencode(id)
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
                        "Received `text/plain` content type response that cannot be converted to `models::GalleryItem`",
                    )));
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be converted to `models::GalleryItem`"
                    ))));
                }
            }
        } else {
            let local_var_entity: Option<GetGalleryWidgetsItemError> =
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

/// struct for typed errors of method [`get_gallery_widget_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGalleryWidgetListError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_gallery_widgets_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGalleryWidgetsItemError {
    Status404(),
    UnknownValue(serde_json::Value),
}
