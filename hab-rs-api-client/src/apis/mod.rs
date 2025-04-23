use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

#[cfg(feature = "actions_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "actions_api")))]
pub mod actions_api;
#[cfg(feature = "addons_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "addons_api")))]
pub mod addons_api;
#[cfg(feature = "audio_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio_api")))]
pub mod audio_api;
#[cfg(feature = "auth_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "auth_api")))]
pub mod auth_api;
#[cfg(feature = "channel_types_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "channel_types_api")))]
pub mod channel_types_api;
#[cfg(feature = "config_descriptions_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "config_descriptions_api")))]
pub mod config_descriptions_api;
#[cfg(feature = "discovery_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "discovery_api")))]
pub mod discovery_api;
#[cfg(feature = "events_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "events_api")))]
pub mod events_api;
#[cfg(feature = "habpanel_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "habpanel_api")))]
pub mod habpanel_api;
#[cfg(feature = "iconsets_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "iconsets_api")))]
pub mod iconsets_api;
#[cfg(feature = "inbox_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "inbox_api")))]
pub mod inbox_api;
#[cfg(feature = "items_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "items_api")))]
pub mod items_api;
#[cfg(feature = "links_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "links_api")))]
pub mod links_api;
#[cfg(feature = "logging_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "logging_api")))]
pub mod logging_api;
#[cfg(feature = "module_types_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "module_types_api")))]
pub mod module_types_api;
#[cfg(feature = "persistence_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "persistence_api")))]
pub mod persistence_api;
#[cfg(feature = "profile_types_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "profile_types_api")))]
pub mod profile_types_api;
#[cfg(feature = "root_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "root_api")))]
pub mod root_api;
#[cfg(feature = "rules_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "rules_api")))]
pub mod rules_api;
#[cfg(feature = "services_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "services_api")))]
pub mod services_api;
#[cfg(feature = "sitemaps_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "sitemaps_api")))]
pub mod sitemaps_api;
#[cfg(feature = "systeminfo_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "systeminfo_api")))]
pub mod systeminfo_api;
#[cfg(feature = "tags_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "tags_api")))]
pub mod tags_api;
#[cfg(feature = "templates_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "templates_api")))]
pub mod templates_api;
#[cfg(feature = "thing_types_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "thing_types_api")))]
pub mod thing_types_api;
#[cfg(feature = "things_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "things_api")))]
pub mod things_api;
#[cfg(feature = "transformations_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "transformations_api")))]
pub mod transformations_api;
#[cfg(feature = "ui_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui_api")))]
pub mod ui_api;
#[cfg(feature = "uuid_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid_api")))]
pub mod uuid_api;
#[cfg(feature = "voice_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "voice_api")))]
pub mod voice_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api: Send + Sync {
    #[cfg(feature = "actions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "actions_api")))]
    fn actions_api(&self) -> &dyn actions_api::ActionsApi;
    #[cfg(feature = "addons_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "addons_api")))]
    fn addons_api(&self) -> &dyn addons_api::AddonsApi;
    #[cfg(feature = "audio_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "audio_api")))]
    fn audio_api(&self) -> &dyn audio_api::AudioApi;
    #[cfg(feature = "auth_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "auth_api")))]
    fn auth_api(&self) -> &dyn auth_api::AuthApi;
    #[cfg(feature = "channel_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "channel_types_api")))]
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi;
    #[cfg(feature = "config_descriptions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "config_descriptions_api")))]
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi;
    #[cfg(feature = "discovery_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "discovery_api")))]
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi;
    #[cfg(feature = "events_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events_api")))]
    fn events_api(&self) -> &dyn events_api::EventsApi;
    #[cfg(feature = "habpanel_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "habpanel_api")))]
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi;
    #[cfg(feature = "iconsets_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "iconsets_api")))]
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi;
    #[cfg(feature = "inbox_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "inbox_api")))]
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi;
    #[cfg(feature = "items_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "items_api")))]
    fn items_api(&self) -> &dyn items_api::ItemsApi;
    #[cfg(feature = "links_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "links_api")))]
    fn links_api(&self) -> &dyn links_api::LinksApi;
    #[cfg(feature = "logging_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "logging_api")))]
    fn logging_api(&self) -> &dyn logging_api::LoggingApi;
    #[cfg(feature = "module_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "module_types_api")))]
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi;
    #[cfg(feature = "persistence_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "persistence_api")))]
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi;
    #[cfg(feature = "profile_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "profile_types_api")))]
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi;
    #[cfg(feature = "root_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "root_api")))]
    fn root_api(&self) -> &dyn root_api::RootApi;
    #[cfg(feature = "rules_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rules_api")))]
    fn rules_api(&self) -> &dyn rules_api::RulesApi;
    #[cfg(feature = "services_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "services_api")))]
    fn services_api(&self) -> &dyn services_api::ServicesApi;
    #[cfg(feature = "sitemaps_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sitemaps_api")))]
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi;
    #[cfg(feature = "systeminfo_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "systeminfo_api")))]
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi;
    #[cfg(feature = "tags_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tags_api")))]
    fn tags_api(&self) -> &dyn tags_api::TagsApi;
    #[cfg(feature = "templates_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "templates_api")))]
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi;
    #[cfg(feature = "thing_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "thing_types_api")))]
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi;
    #[cfg(feature = "things_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "things_api")))]
    fn things_api(&self) -> &dyn things_api::ThingsApi;
    #[cfg(feature = "transformations_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "transformations_api")))]
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi;
    #[cfg(feature = "ui_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ui_api")))]
    fn ui_api(&self) -> &dyn ui_api::UiApi;
    #[cfg(feature = "uuid_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_api")))]
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi;
    #[cfg(feature = "voice_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice_api")))]
    fn voice_api(&self) -> &dyn voice_api::VoiceApi;
}

pub struct ApiClient {
    #[cfg(feature = "actions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "actions_api")))]
    actions_api: Box<dyn actions_api::ActionsApi>,
    #[cfg(feature = "addons_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "addons_api")))]
    addons_api: Box<dyn addons_api::AddonsApi>,
    #[cfg(feature = "audio_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "audio_api")))]
    audio_api: Box<dyn audio_api::AudioApi>,
    #[cfg(feature = "auth_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "auth_api")))]
    auth_api: Box<dyn auth_api::AuthApi>,
    #[cfg(feature = "channel_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "channel_types_api")))]
    channel_types_api: Box<dyn channel_types_api::ChannelTypesApi>,
    #[cfg(feature = "config_descriptions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "config_descriptions_api")))]
    config_descriptions_api: Box<dyn config_descriptions_api::ConfigDescriptionsApi>,
    #[cfg(feature = "discovery_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "discovery_api")))]
    discovery_api: Box<dyn discovery_api::DiscoveryApi>,
    #[cfg(feature = "events_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events_api")))]
    events_api: Box<dyn events_api::EventsApi>,
    #[cfg(feature = "habpanel_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "habpanel_api")))]
    habpanel_api: Box<dyn habpanel_api::HabpanelApi>,
    #[cfg(feature = "iconsets_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "iconsets_api")))]
    iconsets_api: Box<dyn iconsets_api::IconsetsApi>,
    #[cfg(feature = "inbox_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "inbox_api")))]
    inbox_api: Box<dyn inbox_api::InboxApi>,
    #[cfg(feature = "items_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "items_api")))]
    items_api: Box<dyn items_api::ItemsApi>,
    #[cfg(feature = "links_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "links_api")))]
    links_api: Box<dyn links_api::LinksApi>,
    #[cfg(feature = "logging_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "logging_api")))]
    logging_api: Box<dyn logging_api::LoggingApi>,
    #[cfg(feature = "module_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "module_types_api")))]
    module_types_api: Box<dyn module_types_api::ModuleTypesApi>,
    #[cfg(feature = "persistence_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "persistence_api")))]
    persistence_api: Box<dyn persistence_api::PersistenceApi>,
    #[cfg(feature = "profile_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "profile_types_api")))]
    profile_types_api: Box<dyn profile_types_api::ProfileTypesApi>,
    #[cfg(feature = "root_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "root_api")))]
    root_api: Box<dyn root_api::RootApi>,
    #[cfg(feature = "rules_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rules_api")))]
    rules_api: Box<dyn rules_api::RulesApi>,
    #[cfg(feature = "services_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "services_api")))]
    services_api: Box<dyn services_api::ServicesApi>,
    #[cfg(feature = "sitemaps_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sitemaps_api")))]
    sitemaps_api: Box<dyn sitemaps_api::SitemapsApi>,
    #[cfg(feature = "systeminfo_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "systeminfo_api")))]
    systeminfo_api: Box<dyn systeminfo_api::SysteminfoApi>,
    #[cfg(feature = "tags_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tags_api")))]
    tags_api: Box<dyn tags_api::TagsApi>,
    #[cfg(feature = "templates_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "templates_api")))]
    templates_api: Box<dyn templates_api::TemplatesApi>,
    #[cfg(feature = "thing_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "thing_types_api")))]
    thing_types_api: Box<dyn thing_types_api::ThingTypesApi>,
    #[cfg(feature = "things_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "things_api")))]
    things_api: Box<dyn things_api::ThingsApi>,
    #[cfg(feature = "transformations_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "transformations_api")))]
    transformations_api: Box<dyn transformations_api::TransformationsApi>,
    #[cfg(feature = "ui_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ui_api")))]
    ui_api: Box<dyn ui_api::UiApi>,
    #[cfg(feature = "uuid_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_api")))]
    uuid_api: Box<dyn uuid_api::UuidApi>,
    #[cfg(feature = "voice_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice_api")))]
    voice_api: Box<dyn voice_api::VoiceApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            #[cfg(feature = "actions_api")]
            actions_api: Box::new(actions_api::ActionsApiClient::new(configuration.clone())),
            #[cfg(feature = "addons_api")]
            addons_api: Box::new(addons_api::AddonsApiClient::new(configuration.clone())),
            #[cfg(feature = "audio_api")]
            audio_api: Box::new(audio_api::AudioApiClient::new(configuration.clone())),
            #[cfg(feature = "auth_api")]
            auth_api: Box::new(auth_api::AuthApiClient::new(configuration.clone())),
            #[cfg(feature = "channel_types_api")]
            channel_types_api: Box::new(channel_types_api::ChannelTypesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "config_descriptions_api")]
            config_descriptions_api: Box::new(
                config_descriptions_api::ConfigDescriptionsApiClient::new(configuration.clone()),
            ),
            #[cfg(feature = "discovery_api")]
            discovery_api: Box::new(discovery_api::DiscoveryApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "events_api")]
            events_api: Box::new(events_api::EventsApiClient::new(configuration.clone())),
            #[cfg(feature = "habpanel_api")]
            habpanel_api: Box::new(habpanel_api::HabpanelApiClient::new(configuration.clone())),
            #[cfg(feature = "iconsets_api")]
            iconsets_api: Box::new(iconsets_api::IconsetsApiClient::new(configuration.clone())),
            #[cfg(feature = "inbox_api")]
            inbox_api: Box::new(inbox_api::InboxApiClient::new(configuration.clone())),
            #[cfg(feature = "items_api")]
            items_api: Box::new(items_api::ItemsApiClient::new(configuration.clone())),
            #[cfg(feature = "links_api")]
            links_api: Box::new(links_api::LinksApiClient::new(configuration.clone())),
            #[cfg(feature = "logging_api")]
            logging_api: Box::new(logging_api::LoggingApiClient::new(configuration.clone())),
            #[cfg(feature = "module_types_api")]
            module_types_api: Box::new(module_types_api::ModuleTypesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "persistence_api")]
            persistence_api: Box::new(persistence_api::PersistenceApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "profile_types_api")]
            profile_types_api: Box::new(profile_types_api::ProfileTypesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "root_api")]
            root_api: Box::new(root_api::RootApiClient::new(configuration.clone())),
            #[cfg(feature = "rules_api")]
            rules_api: Box::new(rules_api::RulesApiClient::new(configuration.clone())),
            #[cfg(feature = "services_api")]
            services_api: Box::new(services_api::ServicesApiClient::new(configuration.clone())),
            #[cfg(feature = "sitemaps_api")]
            sitemaps_api: Box::new(sitemaps_api::SitemapsApiClient::new(configuration.clone())),
            #[cfg(feature = "systeminfo_api")]
            systeminfo_api: Box::new(systeminfo_api::SysteminfoApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "tags_api")]
            tags_api: Box::new(tags_api::TagsApiClient::new(configuration.clone())),
            #[cfg(feature = "templates_api")]
            templates_api: Box::new(templates_api::TemplatesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "thing_types_api")]
            thing_types_api: Box::new(thing_types_api::ThingTypesApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "things_api")]
            things_api: Box::new(things_api::ThingsApiClient::new(configuration.clone())),
            #[cfg(feature = "transformations_api")]
            transformations_api: Box::new(transformations_api::TransformationsApiClient::new(
                configuration.clone(),
            )),
            #[cfg(feature = "ui_api")]
            ui_api: Box::new(ui_api::UiApiClient::new(configuration.clone())),
            #[cfg(feature = "uuid_api")]
            uuid_api: Box::new(uuid_api::UuidApiClient::new(configuration.clone())),
            #[cfg(feature = "voice_api")]
            voice_api: Box::new(voice_api::VoiceApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    #[cfg(feature = "actions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "actions_api")))]
    fn actions_api(&self) -> &dyn actions_api::ActionsApi {
        self.actions_api.as_ref()
    }
    #[cfg(feature = "addons_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "addons_api")))]
    fn addons_api(&self) -> &dyn addons_api::AddonsApi {
        self.addons_api.as_ref()
    }
    #[cfg(feature = "audio_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "audio_api")))]
    fn audio_api(&self) -> &dyn audio_api::AudioApi {
        self.audio_api.as_ref()
    }
    #[cfg(feature = "auth_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "auth_api")))]
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        self.auth_api.as_ref()
    }
    #[cfg(feature = "channel_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "channel_types_api")))]
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi {
        self.channel_types_api.as_ref()
    }
    #[cfg(feature = "config_descriptions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "config_descriptions_api")))]
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi {
        self.config_descriptions_api.as_ref()
    }
    #[cfg(feature = "discovery_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "discovery_api")))]
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi {
        self.discovery_api.as_ref()
    }
    #[cfg(feature = "events_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events_api")))]
    fn events_api(&self) -> &dyn events_api::EventsApi {
        self.events_api.as_ref()
    }
    #[cfg(feature = "habpanel_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "habpanel_api")))]
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi {
        self.habpanel_api.as_ref()
    }
    #[cfg(feature = "iconsets_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "iconsets_api")))]
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi {
        self.iconsets_api.as_ref()
    }
    #[cfg(feature = "inbox_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "inbox_api")))]
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi {
        self.inbox_api.as_ref()
    }
    #[cfg(feature = "items_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "items_api")))]
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        self.items_api.as_ref()
    }
    #[cfg(feature = "links_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "links_api")))]
    fn links_api(&self) -> &dyn links_api::LinksApi {
        self.links_api.as_ref()
    }
    #[cfg(feature = "logging_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "logging_api")))]
    fn logging_api(&self) -> &dyn logging_api::LoggingApi {
        self.logging_api.as_ref()
    }
    #[cfg(feature = "module_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "module_types_api")))]
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi {
        self.module_types_api.as_ref()
    }
    #[cfg(feature = "persistence_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "persistence_api")))]
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi {
        self.persistence_api.as_ref()
    }
    #[cfg(feature = "profile_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "profile_types_api")))]
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi {
        self.profile_types_api.as_ref()
    }
    #[cfg(feature = "root_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "root_api")))]
    fn root_api(&self) -> &dyn root_api::RootApi {
        self.root_api.as_ref()
    }
    #[cfg(feature = "rules_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rules_api")))]
    fn rules_api(&self) -> &dyn rules_api::RulesApi {
        self.rules_api.as_ref()
    }
    #[cfg(feature = "services_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "services_api")))]
    fn services_api(&self) -> &dyn services_api::ServicesApi {
        self.services_api.as_ref()
    }
    #[cfg(feature = "sitemaps_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sitemaps_api")))]
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi {
        self.sitemaps_api.as_ref()
    }
    #[cfg(feature = "systeminfo_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "systeminfo_api")))]
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi {
        self.systeminfo_api.as_ref()
    }
    #[cfg(feature = "tags_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tags_api")))]
    fn tags_api(&self) -> &dyn tags_api::TagsApi {
        self.tags_api.as_ref()
    }
    #[cfg(feature = "templates_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "templates_api")))]
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi {
        self.templates_api.as_ref()
    }
    #[cfg(feature = "thing_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "thing_types_api")))]
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi {
        self.thing_types_api.as_ref()
    }
    #[cfg(feature = "things_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "things_api")))]
    fn things_api(&self) -> &dyn things_api::ThingsApi {
        self.things_api.as_ref()
    }
    #[cfg(feature = "transformations_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "transformations_api")))]
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi {
        self.transformations_api.as_ref()
    }
    #[cfg(feature = "ui_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ui_api")))]
    fn ui_api(&self) -> &dyn ui_api::UiApi {
        self.ui_api.as_ref()
    }
    #[cfg(feature = "uuid_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid_api")))]
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi {
        self.uuid_api.as_ref()
    }
    #[cfg(feature = "voice_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice_api")))]
    fn voice_api(&self) -> &dyn voice_api::VoiceApi {
        self.voice_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
#[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
pub struct MockApiClient {
    #[cfg(feature = "actions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub actions_api_mock: actions_api::MockActionsApi,
    #[cfg(feature = "addons_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub addons_api_mock: addons_api::MockAddonsApi,
    #[cfg(feature = "audio_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub audio_api_mock: audio_api::MockAudioApi,
    #[cfg(feature = "auth_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub auth_api_mock: auth_api::MockAuthApi,
    #[cfg(feature = "channel_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub channel_types_api_mock: channel_types_api::MockChannelTypesApi,
    #[cfg(feature = "config_descriptions_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub config_descriptions_api_mock: config_descriptions_api::MockConfigDescriptionsApi,
    #[cfg(feature = "discovery_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub discovery_api_mock: discovery_api::MockDiscoveryApi,
    #[cfg(feature = "events_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub events_api_mock: events_api::MockEventsApi,
    #[cfg(feature = "habpanel_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub habpanel_api_mock: habpanel_api::MockHabpanelApi,
    #[cfg(feature = "iconsets_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub iconsets_api_mock: iconsets_api::MockIconsetsApi,
    #[cfg(feature = "inbox_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub inbox_api_mock: inbox_api::MockInboxApi,
    #[cfg(feature = "items_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub items_api_mock: items_api::MockItemsApi,
    #[cfg(feature = "links_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub links_api_mock: links_api::MockLinksApi,
    #[cfg(feature = "logging_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub logging_api_mock: logging_api::MockLoggingApi,
    #[cfg(feature = "module_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub module_types_api_mock: module_types_api::MockModuleTypesApi,
    #[cfg(feature = "persistence_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub persistence_api_mock: persistence_api::MockPersistenceApi,
    #[cfg(feature = "profile_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub profile_types_api_mock: profile_types_api::MockProfileTypesApi,
    #[cfg(feature = "root_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub root_api_mock: root_api::MockRootApi,
    #[cfg(feature = "rules_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub rules_api_mock: rules_api::MockRulesApi,
    #[cfg(feature = "services_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub services_api_mock: services_api::MockServicesApi,
    #[cfg(feature = "sitemaps_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub sitemaps_api_mock: sitemaps_api::MockSitemapsApi,
    #[cfg(feature = "systeminfo_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub systeminfo_api_mock: systeminfo_api::MockSysteminfoApi,
    #[cfg(feature = "tags_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub tags_api_mock: tags_api::MockTagsApi,
    #[cfg(feature = "templates_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub templates_api_mock: templates_api::MockTemplatesApi,
    #[cfg(feature = "thing_types_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub thing_types_api_mock: thing_types_api::MockThingTypesApi,
    #[cfg(feature = "things_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub things_api_mock: things_api::MockThingsApi,
    #[cfg(feature = "transformations_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub transformations_api_mock: transformations_api::MockTransformationsApi,
    #[cfg(feature = "ui_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub ui_api_mock: ui_api::MockUiApi,
    #[cfg(feature = "uuid_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub uuid_api_mock: uuid_api::MockUuidApi,
    #[cfg(feature = "voice_api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
    pub voice_api_mock: voice_api::MockVoiceApi,
}

#[cfg(feature = "mockall")]
#[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "actions_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            actions_api_mock: actions_api::MockActionsApi::new(),
            #[cfg(feature = "addons_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            addons_api_mock: addons_api::MockAddonsApi::new(),
            #[cfg(feature = "audio_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            audio_api_mock: audio_api::MockAudioApi::new(),
            #[cfg(feature = "auth_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            auth_api_mock: auth_api::MockAuthApi::new(),
            #[cfg(feature = "channel_types_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            channel_types_api_mock: channel_types_api::MockChannelTypesApi::new(),
            #[cfg(feature = "config_descriptions_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            config_descriptions_api_mock: config_descriptions_api::MockConfigDescriptionsApi::new(),
            #[cfg(feature = "discovery_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            discovery_api_mock: discovery_api::MockDiscoveryApi::new(),
            #[cfg(feature = "events_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            events_api_mock: events_api::MockEventsApi::new(),
            #[cfg(feature = "habpanel_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            habpanel_api_mock: habpanel_api::MockHabpanelApi::new(),
            #[cfg(feature = "iconsets_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            iconsets_api_mock: iconsets_api::MockIconsetsApi::new(),
            #[cfg(feature = "inbox_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            inbox_api_mock: inbox_api::MockInboxApi::new(),
            #[cfg(feature = "items_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            items_api_mock: items_api::MockItemsApi::new(),
            #[cfg(feature = "links_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            links_api_mock: links_api::MockLinksApi::new(),
            #[cfg(feature = "logging_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            logging_api_mock: logging_api::MockLoggingApi::new(),
            #[cfg(feature = "module_types_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            module_types_api_mock: module_types_api::MockModuleTypesApi::new(),
            #[cfg(feature = "persistence_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            persistence_api_mock: persistence_api::MockPersistenceApi::new(),
            #[cfg(feature = "profile_types_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            profile_types_api_mock: profile_types_api::MockProfileTypesApi::new(),
            #[cfg(feature = "root_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            root_api_mock: root_api::MockRootApi::new(),
            #[cfg(feature = "rules_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            rules_api_mock: rules_api::MockRulesApi::new(),
            #[cfg(feature = "services_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            services_api_mock: services_api::MockServicesApi::new(),
            #[cfg(feature = "sitemaps_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            sitemaps_api_mock: sitemaps_api::MockSitemapsApi::new(),
            #[cfg(feature = "systeminfo_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            systeminfo_api_mock: systeminfo_api::MockSysteminfoApi::new(),
            #[cfg(feature = "tags_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            tags_api_mock: tags_api::MockTagsApi::new(),
            #[cfg(feature = "templates_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            templates_api_mock: templates_api::MockTemplatesApi::new(),
            #[cfg(feature = "thing_types_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            thing_types_api_mock: thing_types_api::MockThingTypesApi::new(),
            #[cfg(feature = "things_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            things_api_mock: things_api::MockThingsApi::new(),
            #[cfg(feature = "transformations_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            transformations_api_mock: transformations_api::MockTransformationsApi::new(),
            #[cfg(feature = "ui_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            ui_api_mock: ui_api::MockUiApi::new(),
            #[cfg(feature = "uuid_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            uuid_api_mock: uuid_api::MockUuidApi::new(),
            #[cfg(feature = "voice_api")]
            #[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
            voice_api_mock: voice_api::MockVoiceApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
#[cfg_attr(docsrs, doc(cfg(feature = "mockall")))]
impl Api for MockApiClient {
    #[cfg(feature = "actions_api")]
    fn actions_api(&self) -> &dyn actions_api::ActionsApi {
        &self.actions_api_mock
    }
    #[cfg(feature = "addons_api")]
    fn addons_api(&self) -> &dyn addons_api::AddonsApi {
        &self.addons_api_mock
    }
    #[cfg(feature = "audio_api")]
    fn audio_api(&self) -> &dyn audio_api::AudioApi {
        &self.audio_api_mock
    }
    #[cfg(feature = "auth_api")]
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        &self.auth_api_mock
    }
    #[cfg(feature = "channel_types_api")]
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi {
        &self.channel_types_api_mock
    }
    #[cfg(feature = "config_descriptions_api")]
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi {
        &self.config_descriptions_api_mock
    }
    #[cfg(feature = "discovery_api")]
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi {
        &self.discovery_api_mock
    }
    #[cfg(feature = "events_api")]
    fn events_api(&self) -> &dyn events_api::EventsApi {
        &self.events_api_mock
    }
    #[cfg(feature = "habpanel_api")]
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi {
        &self.habpanel_api_mock
    }
    #[cfg(feature = "iconsets_api")]
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi {
        &self.iconsets_api_mock
    }
    #[cfg(feature = "inbox_api")]
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi {
        &self.inbox_api_mock
    }
    #[cfg(feature = "items_api")]
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        &self.items_api_mock
    }
    #[cfg(feature = "links_api")]
    fn links_api(&self) -> &dyn links_api::LinksApi {
        &self.links_api_mock
    }
    #[cfg(feature = "logging_api")]
    fn logging_api(&self) -> &dyn logging_api::LoggingApi {
        &self.logging_api_mock
    }
    #[cfg(feature = "module_types_api")]
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi {
        &self.module_types_api_mock
    }
    #[cfg(feature = "persistence_api")]
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi {
        &self.persistence_api_mock
    }
    #[cfg(feature = "profile_types_api")]
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi {
        &self.profile_types_api_mock
    }
    #[cfg(feature = "root_api")]
    fn root_api(&self) -> &dyn root_api::RootApi {
        &self.root_api_mock
    }
    #[cfg(feature = "rules_api")]
    fn rules_api(&self) -> &dyn rules_api::RulesApi {
        &self.rules_api_mock
    }
    #[cfg(feature = "services_api")]
    fn services_api(&self) -> &dyn services_api::ServicesApi {
        &self.services_api_mock
    }
    #[cfg(feature = "sitemaps_api")]
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi {
        &self.sitemaps_api_mock
    }
    #[cfg(feature = "systeminfo_api")]
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi {
        &self.systeminfo_api_mock
    }
    #[cfg(feature = "tags_api")]
    fn tags_api(&self) -> &dyn tags_api::TagsApi {
        &self.tags_api_mock
    }
    #[cfg(feature = "templates_api")]
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi {
        &self.templates_api_mock
    }
    #[cfg(feature = "thing_types_api")]
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi {
        &self.thing_types_api_mock
    }
    #[cfg(feature = "things_api")]
    fn things_api(&self) -> &dyn things_api::ThingsApi {
        &self.things_api_mock
    }
    #[cfg(feature = "transformations_api")]
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi {
        &self.transformations_api_mock
    }
    #[cfg(feature = "ui_api")]
    fn ui_api(&self) -> &dyn ui_api::UiApi {
        &self.ui_api_mock
    }
    #[cfg(feature = "uuid_api")]
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi {
        &self.uuid_api_mock
    }
    #[cfg(feature = "voice_api")]
    fn voice_api(&self) -> &dyn voice_api::VoiceApi {
        &self.voice_api_mock
    }
}
