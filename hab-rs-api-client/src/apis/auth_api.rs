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
pub trait AuthApi: Send + Sync {
    /// POST /auth/logout
    ///
    ///
    async fn delete_session<'id, 'refresh_token>(
        &self,
        id: Option<&'id str>,
        refresh_token: Option<&'refresh_token str>,
    ) -> Result<(), Error<DeleteSessionError>>;

    /// GET /auth/apitokens
    ///
    ///
    async fn get_api_tokens(
        &self,
    ) -> Result<Vec<models::UserApiTokenDto>, Error<GetApiTokensError>>;

    /// POST /auth/token
    ///
    ///
    async fn get_o_auth_token<
        'use_cookie,
        'client_id,
        'code,
        'code_verifier,
        'grant_type,
        'redirect_uri,
        'refresh_token,
    >(
        &self,
        use_cookie: Option<bool>,
        client_id: Option<&'client_id str>,
        code: Option<&'code str>,
        code_verifier: Option<&'code_verifier str>,
        grant_type: Option<&'grant_type str>,
        redirect_uri: Option<&'redirect_uri str>,
        refresh_token: Option<&'refresh_token str>,
    ) -> Result<models::TokenResponseDto, Error<GetOAuthTokenError>>;

    /// GET /auth/sessions
    ///
    ///
    async fn get_sessions_for_current_user(
        &self,
    ) -> Result<Vec<models::UserSessionDto>, Error<GetSessionsForCurrentUserError>>;

    /// DELETE /auth/apitokens/{name}
    ///
    ///
    async fn remove_api_token<'name>(
        &self,
        name: &'name str,
    ) -> Result<(), Error<RemoveApiTokenError>>;
}

pub struct AuthApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl AuthApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl AuthApi for AuthApiClient {
    async fn delete_session<'id, 'refresh_token>(
        &self,
        id: Option<&'id str>,
        refresh_token: Option<&'refresh_token str>,
    ) -> Result<(), Error<DeleteSessionError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/auth/logout", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = id {
            local_var_form_params.insert("id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = refresh_token {
            local_var_form_params.insert("refresh_token", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteSessionError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_api_tokens(
        &self,
    ) -> Result<Vec<models::UserApiTokenDto>, Error<GetApiTokensError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/auth/apitokens", local_var_configuration.base_path);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::UserApiTokenDto&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::UserApiTokenDto&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetApiTokensError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_o_auth_token<
        'use_cookie,
        'client_id,
        'code,
        'code_verifier,
        'grant_type,
        'redirect_uri,
        'refresh_token,
    >(
        &self,
        use_cookie: Option<bool>,
        client_id: Option<&'client_id str>,
        code: Option<&'code str>,
        code_verifier: Option<&'code_verifier str>,
        grant_type: Option<&'grant_type str>,
        redirect_uri: Option<&'redirect_uri str>,
        refresh_token: Option<&'refresh_token str>,
    ) -> Result<models::TokenResponseDto, Error<GetOAuthTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/auth/token", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = use_cookie {
            local_var_req_builder =
                local_var_req_builder.query(&[("useCookie", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        let mut local_var_form_params = std::collections::HashMap::new();
        if let Some(local_var_param_value) = client_id {
            local_var_form_params.insert("client_id", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = code {
            local_var_form_params.insert("code", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = code_verifier {
            local_var_form_params.insert("code_verifier", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = grant_type {
            local_var_form_params.insert("grant_type", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = redirect_uri {
            local_var_form_params.insert("redirect_uri", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = refresh_token {
            local_var_form_params.insert("refresh_token", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TokenResponseDto`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `models::TokenResponseDto`")))),
            }
        } else {
            let local_var_entity: Option<GetOAuthTokenError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn get_sessions_for_current_user(
        &self,
    ) -> Result<Vec<models::UserSessionDto>, Error<GetSessionsForCurrentUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/auth/sessions", local_var_configuration.base_path);
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
                ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::UserSessionDto&gt;`"))),
                ContentType::Unsupported(local_var_unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{local_var_unknown_type}` content type response that cannot be converted to `Vec&lt;models::UserSessionDto&gt;`")))),
            }
        } else {
            let local_var_entity: Option<GetSessionsForCurrentUserError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn remove_api_token<'name>(
        &self,
        name: &'name str,
    ) -> Result<(), Error<RemoveApiTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/auth/apitokens/{name}",
            local_var_configuration.base_path,
            name = crate::apis::urlencode(name)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<RemoveApiTokenError> =
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

/// struct for typed errors of method [`delete_session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSessionError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_api_tokens`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiTokensError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_o_auth_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOAuthTokenError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sessions_for_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSessionsForCurrentUserError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_api_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveApiTokenError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}
