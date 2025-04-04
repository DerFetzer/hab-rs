/*
 * openHAB REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 8
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
use crate::apis::ContentType;

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait SitemapsApi: Send + Sync {

    /// POST /sitemaps/events/subscribe
    ///
    /// 
    async fn create_sitemap_event_subscription<>(&self, ) -> Result<(), Error<CreateSitemapEventSubscriptionError>>;

    /// GET /sitemaps/{sitemapname}
    ///
    /// 
    async fn get_sitemap_by_name<'sitemapname, 'accept_language, 'r_type, 'jsoncallback, 'include_hidden>(&self, sitemapname: &'sitemapname str, accept_language: Option<&'accept_language str>, r#type: Option<&'r_type str>, jsoncallback: Option<&'jsoncallback str>, include_hidden: Option<bool>) -> Result<models::SitemapDto, Error<GetSitemapByNameError>>;

    /// GET /sitemaps/events/{subscriptionid}/*
    ///
    /// 
    async fn get_sitemap_events<'subscriptionid, 'sitemap>(&self, subscriptionid: &'subscriptionid str, sitemap: Option<&'sitemap str>) -> Result<(), Error<GetSitemapEventsError>>;

    /// GET /sitemaps/events/{subscriptionid}
    ///
    /// 
    async fn get_sitemap_events1<'subscriptionid, 'sitemap, 'pageid>(&self, subscriptionid: &'subscriptionid str, sitemap: Option<&'sitemap str>, pageid: Option<&'pageid str>) -> Result<(), Error<GetSitemapEvents1Error>>;

    /// GET /sitemaps
    ///
    /// 
    async fn get_sitemaps<>(&self, ) -> Result<Vec<models::SitemapDto>, Error<GetSitemapsError>>;

    /// GET /sitemaps/{sitemapname}/{pageid}
    ///
    /// 
    async fn poll_data_for_page<'sitemapname, 'pageid, 'accept_language, 'subscriptionid, 'include_hidden>(&self, sitemapname: &'sitemapname str, pageid: &'pageid str, accept_language: Option<&'accept_language str>, subscriptionid: Option<&'subscriptionid str>, include_hidden: Option<bool>) -> Result<models::PageDto, Error<PollDataForPageError>>;

    /// GET /sitemaps/{sitemapname}/*
    ///
    /// 
    async fn poll_data_for_sitemap<'sitemapname, 'accept_language, 'subscriptionid, 'include_hidden>(&self, sitemapname: &'sitemapname str, accept_language: Option<&'accept_language str>, subscriptionid: Option<&'subscriptionid str>, include_hidden: Option<bool>) -> Result<models::SitemapDto, Error<PollDataForSitemapError>>;
}

pub struct SitemapsApiClient {
    configuration: Arc<configuration::Configuration>
}

impl SitemapsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl SitemapsApi for SitemapsApiClient {
    async fn create_sitemap_event_subscription<>(&self, ) -> Result<(), Error<CreateSitemapEventSubscriptionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/events/subscribe", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<CreateSitemapEventSubscriptionError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sitemap_by_name<'sitemapname, 'accept_language, 'r_type, 'jsoncallback, 'include_hidden>(&self, sitemapname: &'sitemapname str, accept_language: Option<&'accept_language str>, r#type: Option<&'r_type str>, jsoncallback: Option<&'jsoncallback str>, include_hidden: Option<bool>) -> Result<models::SitemapDto, Error<GetSitemapByNameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/{sitemapname}", local_var_configuration.base_path, sitemapname=crate::apis::urlencode(sitemapname));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = r#type {
            local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = jsoncallback {
            local_var_req_builder = local_var_req_builder.query(&[("jsoncallback", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include_hidden {
            local_var_req_builder = local_var_req_builder.query(&[("includeHidden", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SitemapDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::SitemapDto`")))),
            }
        } else {
            let local_var_entity: Option<GetSitemapByNameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sitemap_events<'subscriptionid, 'sitemap>(&self, subscriptionid: &'subscriptionid str, sitemap: Option<&'sitemap str>) -> Result<(), Error<GetSitemapEventsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/events/{subscriptionid}/*", local_var_configuration.base_path, subscriptionid=crate::apis::urlencode(subscriptionid));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = sitemap {
            local_var_req_builder = local_var_req_builder.query(&[("sitemap", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<GetSitemapEventsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sitemap_events1<'subscriptionid, 'sitemap, 'pageid>(&self, subscriptionid: &'subscriptionid str, sitemap: Option<&'sitemap str>, pageid: Option<&'pageid str>) -> Result<(), Error<GetSitemapEvents1Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/events/{subscriptionid}", local_var_configuration.base_path, subscriptionid=crate::apis::urlencode(subscriptionid));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = sitemap {
            local_var_req_builder = local_var_req_builder.query(&[("sitemap", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = pageid {
            local_var_req_builder = local_var_req_builder.query(&[("pageid", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<GetSitemapEvents1Error> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sitemaps<>(&self, ) -> Result<Vec<models::SitemapDto>, Error<GetSitemapsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::SitemapDto&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::SitemapDto&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetSitemapsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn poll_data_for_page<'sitemapname, 'pageid, 'accept_language, 'subscriptionid, 'include_hidden>(&self, sitemapname: &'sitemapname str, pageid: &'pageid str, accept_language: Option<&'accept_language str>, subscriptionid: Option<&'subscriptionid str>, include_hidden: Option<bool>) -> Result<models::PageDto, Error<PollDataForPageError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/{sitemapname}/{pageid}", local_var_configuration.base_path, sitemapname=crate::apis::urlencode(sitemapname), pageid=crate::apis::urlencode(pageid));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = subscriptionid {
            local_var_req_builder = local_var_req_builder.query(&[("subscriptionid", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include_hidden {
            local_var_req_builder = local_var_req_builder.query(&[("includeHidden", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PageDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::PageDto`")))),
            }
        } else {
            let local_var_entity: Option<PollDataForPageError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn poll_data_for_sitemap<'sitemapname, 'accept_language, 'subscriptionid, 'include_hidden>(&self, sitemapname: &'sitemapname str, accept_language: Option<&'accept_language str>, subscriptionid: Option<&'subscriptionid str>, include_hidden: Option<bool>) -> Result<models::SitemapDto, Error<PollDataForSitemapError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/sitemaps/{sitemapname}/*", local_var_configuration.base_path, sitemapname=crate::apis::urlencode(sitemapname));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = subscriptionid {
            local_var_req_builder = local_var_req_builder.query(&[("subscriptionid", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include_hidden {
            local_var_req_builder = local_var_req_builder.query(&[("includeHidden", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = accept_language {
            local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SitemapDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::SitemapDto`")))),
            }
        } else {
            let local_var_entity: Option<PollDataForSitemapError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_sitemap_event_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSitemapEventSubscriptionError {
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sitemap_by_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSitemapByNameError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sitemap_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSitemapEventsError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sitemap_events1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSitemapEvents1Error {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sitemaps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSitemapsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`poll_data_for_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollDataForPageError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`poll_data_for_sitemap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollDataForSitemapError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

