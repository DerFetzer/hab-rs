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

impl <T> fmt::Display for Error<T> {
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

impl <T: fmt::Debug> error::Error for Error<T> {
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

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
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
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
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
    Unsupported(String)
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

pub mod actions_api;
pub mod addons_api;
pub mod audio_api;
pub mod auth_api;
pub mod channel_types_api;
pub mod config_descriptions_api;
pub mod discovery_api;
pub mod events_api;
pub mod habpanel_api;
pub mod iconsets_api;
pub mod inbox_api;
pub mod items_api;
pub mod links_api;
pub mod logging_api;
pub mod module_types_api;
pub mod persistence_api;
pub mod profile_types_api;
pub mod root_api;
pub mod rules_api;
pub mod services_api;
pub mod sitemaps_api;
pub mod systeminfo_api;
pub mod tags_api;
pub mod templates_api;
pub mod thing_types_api;
pub mod things_api;
pub mod transformations_api;
pub mod ui_api;
pub mod uuid_api;
pub mod voice_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn actions_api(&self) -> &dyn actions_api::ActionsApi;
    fn addons_api(&self) -> &dyn addons_api::AddonsApi;
    fn audio_api(&self) -> &dyn audio_api::AudioApi;
    fn auth_api(&self) -> &dyn auth_api::AuthApi;
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi;
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi;
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi;
    fn events_api(&self) -> &dyn events_api::EventsApi;
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi;
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi;
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi;
    fn items_api(&self) -> &dyn items_api::ItemsApi;
    fn links_api(&self) -> &dyn links_api::LinksApi;
    fn logging_api(&self) -> &dyn logging_api::LoggingApi;
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi;
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi;
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi;
    fn root_api(&self) -> &dyn root_api::RootApi;
    fn rules_api(&self) -> &dyn rules_api::RulesApi;
    fn services_api(&self) -> &dyn services_api::ServicesApi;
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi;
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi;
    fn tags_api(&self) -> &dyn tags_api::TagsApi;
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi;
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi;
    fn things_api(&self) -> &dyn things_api::ThingsApi;
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi;
    fn ui_api(&self) -> &dyn ui_api::UiApi;
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi;
    fn voice_api(&self) -> &dyn voice_api::VoiceApi;
}

pub struct ApiClient {
    actions_api: Box<dyn actions_api::ActionsApi>,
    addons_api: Box<dyn addons_api::AddonsApi>,
    audio_api: Box<dyn audio_api::AudioApi>,
    auth_api: Box<dyn auth_api::AuthApi>,
    channel_types_api: Box<dyn channel_types_api::ChannelTypesApi>,
    config_descriptions_api: Box<dyn config_descriptions_api::ConfigDescriptionsApi>,
    discovery_api: Box<dyn discovery_api::DiscoveryApi>,
    events_api: Box<dyn events_api::EventsApi>,
    habpanel_api: Box<dyn habpanel_api::HabpanelApi>,
    iconsets_api: Box<dyn iconsets_api::IconsetsApi>,
    inbox_api: Box<dyn inbox_api::InboxApi>,
    items_api: Box<dyn items_api::ItemsApi>,
    links_api: Box<dyn links_api::LinksApi>,
    logging_api: Box<dyn logging_api::LoggingApi>,
    module_types_api: Box<dyn module_types_api::ModuleTypesApi>,
    persistence_api: Box<dyn persistence_api::PersistenceApi>,
    profile_types_api: Box<dyn profile_types_api::ProfileTypesApi>,
    root_api: Box<dyn root_api::RootApi>,
    rules_api: Box<dyn rules_api::RulesApi>,
    services_api: Box<dyn services_api::ServicesApi>,
    sitemaps_api: Box<dyn sitemaps_api::SitemapsApi>,
    systeminfo_api: Box<dyn systeminfo_api::SysteminfoApi>,
    tags_api: Box<dyn tags_api::TagsApi>,
    templates_api: Box<dyn templates_api::TemplatesApi>,
    thing_types_api: Box<dyn thing_types_api::ThingTypesApi>,
    things_api: Box<dyn things_api::ThingsApi>,
    transformations_api: Box<dyn transformations_api::TransformationsApi>,
    ui_api: Box<dyn ui_api::UiApi>,
    uuid_api: Box<dyn uuid_api::UuidApi>,
    voice_api: Box<dyn voice_api::VoiceApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            actions_api: Box::new(actions_api::ActionsApiClient::new(configuration.clone())),
            addons_api: Box::new(addons_api::AddonsApiClient::new(configuration.clone())),
            audio_api: Box::new(audio_api::AudioApiClient::new(configuration.clone())),
            auth_api: Box::new(auth_api::AuthApiClient::new(configuration.clone())),
            channel_types_api: Box::new(channel_types_api::ChannelTypesApiClient::new(configuration.clone())),
            config_descriptions_api: Box::new(config_descriptions_api::ConfigDescriptionsApiClient::new(configuration.clone())),
            discovery_api: Box::new(discovery_api::DiscoveryApiClient::new(configuration.clone())),
            events_api: Box::new(events_api::EventsApiClient::new(configuration.clone())),
            habpanel_api: Box::new(habpanel_api::HabpanelApiClient::new(configuration.clone())),
            iconsets_api: Box::new(iconsets_api::IconsetsApiClient::new(configuration.clone())),
            inbox_api: Box::new(inbox_api::InboxApiClient::new(configuration.clone())),
            items_api: Box::new(items_api::ItemsApiClient::new(configuration.clone())),
            links_api: Box::new(links_api::LinksApiClient::new(configuration.clone())),
            logging_api: Box::new(logging_api::LoggingApiClient::new(configuration.clone())),
            module_types_api: Box::new(module_types_api::ModuleTypesApiClient::new(configuration.clone())),
            persistence_api: Box::new(persistence_api::PersistenceApiClient::new(configuration.clone())),
            profile_types_api: Box::new(profile_types_api::ProfileTypesApiClient::new(configuration.clone())),
            root_api: Box::new(root_api::RootApiClient::new(configuration.clone())),
            rules_api: Box::new(rules_api::RulesApiClient::new(configuration.clone())),
            services_api: Box::new(services_api::ServicesApiClient::new(configuration.clone())),
            sitemaps_api: Box::new(sitemaps_api::SitemapsApiClient::new(configuration.clone())),
            systeminfo_api: Box::new(systeminfo_api::SysteminfoApiClient::new(configuration.clone())),
            tags_api: Box::new(tags_api::TagsApiClient::new(configuration.clone())),
            templates_api: Box::new(templates_api::TemplatesApiClient::new(configuration.clone())),
            thing_types_api: Box::new(thing_types_api::ThingTypesApiClient::new(configuration.clone())),
            things_api: Box::new(things_api::ThingsApiClient::new(configuration.clone())),
            transformations_api: Box::new(transformations_api::TransformationsApiClient::new(configuration.clone())),
            ui_api: Box::new(ui_api::UiApiClient::new(configuration.clone())),
            uuid_api: Box::new(uuid_api::UuidApiClient::new(configuration.clone())),
            voice_api: Box::new(voice_api::VoiceApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn actions_api(&self) -> &dyn actions_api::ActionsApi {
        self.actions_api.as_ref()
    }
    fn addons_api(&self) -> &dyn addons_api::AddonsApi {
        self.addons_api.as_ref()
    }
    fn audio_api(&self) -> &dyn audio_api::AudioApi {
        self.audio_api.as_ref()
    }
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        self.auth_api.as_ref()
    }
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi {
        self.channel_types_api.as_ref()
    }
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi {
        self.config_descriptions_api.as_ref()
    }
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi {
        self.discovery_api.as_ref()
    }
    fn events_api(&self) -> &dyn events_api::EventsApi {
        self.events_api.as_ref()
    }
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi {
        self.habpanel_api.as_ref()
    }
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi {
        self.iconsets_api.as_ref()
    }
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi {
        self.inbox_api.as_ref()
    }
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        self.items_api.as_ref()
    }
    fn links_api(&self) -> &dyn links_api::LinksApi {
        self.links_api.as_ref()
    }
    fn logging_api(&self) -> &dyn logging_api::LoggingApi {
        self.logging_api.as_ref()
    }
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi {
        self.module_types_api.as_ref()
    }
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi {
        self.persistence_api.as_ref()
    }
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi {
        self.profile_types_api.as_ref()
    }
    fn root_api(&self) -> &dyn root_api::RootApi {
        self.root_api.as_ref()
    }
    fn rules_api(&self) -> &dyn rules_api::RulesApi {
        self.rules_api.as_ref()
    }
    fn services_api(&self) -> &dyn services_api::ServicesApi {
        self.services_api.as_ref()
    }
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi {
        self.sitemaps_api.as_ref()
    }
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi {
        self.systeminfo_api.as_ref()
    }
    fn tags_api(&self) -> &dyn tags_api::TagsApi {
        self.tags_api.as_ref()
    }
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi {
        self.templates_api.as_ref()
    }
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi {
        self.thing_types_api.as_ref()
    }
    fn things_api(&self) -> &dyn things_api::ThingsApi {
        self.things_api.as_ref()
    }
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi {
        self.transformations_api.as_ref()
    }
    fn ui_api(&self) -> &dyn ui_api::UiApi {
        self.ui_api.as_ref()
    }
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi {
        self.uuid_api.as_ref()
    }
    fn voice_api(&self) -> &dyn voice_api::VoiceApi {
        self.voice_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub actions_api_mock: actions_api::MockActionsApi,
    pub addons_api_mock: addons_api::MockAddonsApi,
    pub audio_api_mock: audio_api::MockAudioApi,
    pub auth_api_mock: auth_api::MockAuthApi,
    pub channel_types_api_mock: channel_types_api::MockChannelTypesApi,
    pub config_descriptions_api_mock: config_descriptions_api::MockConfigDescriptionsApi,
    pub discovery_api_mock: discovery_api::MockDiscoveryApi,
    pub events_api_mock: events_api::MockEventsApi,
    pub habpanel_api_mock: habpanel_api::MockHabpanelApi,
    pub iconsets_api_mock: iconsets_api::MockIconsetsApi,
    pub inbox_api_mock: inbox_api::MockInboxApi,
    pub items_api_mock: items_api::MockItemsApi,
    pub links_api_mock: links_api::MockLinksApi,
    pub logging_api_mock: logging_api::MockLoggingApi,
    pub module_types_api_mock: module_types_api::MockModuleTypesApi,
    pub persistence_api_mock: persistence_api::MockPersistenceApi,
    pub profile_types_api_mock: profile_types_api::MockProfileTypesApi,
    pub root_api_mock: root_api::MockRootApi,
    pub rules_api_mock: rules_api::MockRulesApi,
    pub services_api_mock: services_api::MockServicesApi,
    pub sitemaps_api_mock: sitemaps_api::MockSitemapsApi,
    pub systeminfo_api_mock: systeminfo_api::MockSysteminfoApi,
    pub tags_api_mock: tags_api::MockTagsApi,
    pub templates_api_mock: templates_api::MockTemplatesApi,
    pub thing_types_api_mock: thing_types_api::MockThingTypesApi,
    pub things_api_mock: things_api::MockThingsApi,
    pub transformations_api_mock: transformations_api::MockTransformationsApi,
    pub ui_api_mock: ui_api::MockUiApi,
    pub uuid_api_mock: uuid_api::MockUuidApi,
    pub voice_api_mock: voice_api::MockVoiceApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            actions_api_mock: actions_api::MockActionsApi::new(),
            addons_api_mock: addons_api::MockAddonsApi::new(),
            audio_api_mock: audio_api::MockAudioApi::new(),
            auth_api_mock: auth_api::MockAuthApi::new(),
            channel_types_api_mock: channel_types_api::MockChannelTypesApi::new(),
            config_descriptions_api_mock: config_descriptions_api::MockConfigDescriptionsApi::new(),
            discovery_api_mock: discovery_api::MockDiscoveryApi::new(),
            events_api_mock: events_api::MockEventsApi::new(),
            habpanel_api_mock: habpanel_api::MockHabpanelApi::new(),
            iconsets_api_mock: iconsets_api::MockIconsetsApi::new(),
            inbox_api_mock: inbox_api::MockInboxApi::new(),
            items_api_mock: items_api::MockItemsApi::new(),
            links_api_mock: links_api::MockLinksApi::new(),
            logging_api_mock: logging_api::MockLoggingApi::new(),
            module_types_api_mock: module_types_api::MockModuleTypesApi::new(),
            persistence_api_mock: persistence_api::MockPersistenceApi::new(),
            profile_types_api_mock: profile_types_api::MockProfileTypesApi::new(),
            root_api_mock: root_api::MockRootApi::new(),
            rules_api_mock: rules_api::MockRulesApi::new(),
            services_api_mock: services_api::MockServicesApi::new(),
            sitemaps_api_mock: sitemaps_api::MockSitemapsApi::new(),
            systeminfo_api_mock: systeminfo_api::MockSysteminfoApi::new(),
            tags_api_mock: tags_api::MockTagsApi::new(),
            templates_api_mock: templates_api::MockTemplatesApi::new(),
            thing_types_api_mock: thing_types_api::MockThingTypesApi::new(),
            things_api_mock: things_api::MockThingsApi::new(),
            transformations_api_mock: transformations_api::MockTransformationsApi::new(),
            ui_api_mock: ui_api::MockUiApi::new(),
            uuid_api_mock: uuid_api::MockUuidApi::new(),
            voice_api_mock: voice_api::MockVoiceApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn actions_api(&self) -> &dyn actions_api::ActionsApi {
        &self.actions_api_mock
    }
    fn addons_api(&self) -> &dyn addons_api::AddonsApi {
        &self.addons_api_mock
    }
    fn audio_api(&self) -> &dyn audio_api::AudioApi {
        &self.audio_api_mock
    }
    fn auth_api(&self) -> &dyn auth_api::AuthApi {
        &self.auth_api_mock
    }
    fn channel_types_api(&self) -> &dyn channel_types_api::ChannelTypesApi {
        &self.channel_types_api_mock
    }
    fn config_descriptions_api(&self) -> &dyn config_descriptions_api::ConfigDescriptionsApi {
        &self.config_descriptions_api_mock
    }
    fn discovery_api(&self) -> &dyn discovery_api::DiscoveryApi {
        &self.discovery_api_mock
    }
    fn events_api(&self) -> &dyn events_api::EventsApi {
        &self.events_api_mock
    }
    fn habpanel_api(&self) -> &dyn habpanel_api::HabpanelApi {
        &self.habpanel_api_mock
    }
    fn iconsets_api(&self) -> &dyn iconsets_api::IconsetsApi {
        &self.iconsets_api_mock
    }
    fn inbox_api(&self) -> &dyn inbox_api::InboxApi {
        &self.inbox_api_mock
    }
    fn items_api(&self) -> &dyn items_api::ItemsApi {
        &self.items_api_mock
    }
    fn links_api(&self) -> &dyn links_api::LinksApi {
        &self.links_api_mock
    }
    fn logging_api(&self) -> &dyn logging_api::LoggingApi {
        &self.logging_api_mock
    }
    fn module_types_api(&self) -> &dyn module_types_api::ModuleTypesApi {
        &self.module_types_api_mock
    }
    fn persistence_api(&self) -> &dyn persistence_api::PersistenceApi {
        &self.persistence_api_mock
    }
    fn profile_types_api(&self) -> &dyn profile_types_api::ProfileTypesApi {
        &self.profile_types_api_mock
    }
    fn root_api(&self) -> &dyn root_api::RootApi {
        &self.root_api_mock
    }
    fn rules_api(&self) -> &dyn rules_api::RulesApi {
        &self.rules_api_mock
    }
    fn services_api(&self) -> &dyn services_api::ServicesApi {
        &self.services_api_mock
    }
    fn sitemaps_api(&self) -> &dyn sitemaps_api::SitemapsApi {
        &self.sitemaps_api_mock
    }
    fn systeminfo_api(&self) -> &dyn systeminfo_api::SysteminfoApi {
        &self.systeminfo_api_mock
    }
    fn tags_api(&self) -> &dyn tags_api::TagsApi {
        &self.tags_api_mock
    }
    fn templates_api(&self) -> &dyn templates_api::TemplatesApi {
        &self.templates_api_mock
    }
    fn thing_types_api(&self) -> &dyn thing_types_api::ThingTypesApi {
        &self.thing_types_api_mock
    }
    fn things_api(&self) -> &dyn things_api::ThingsApi {
        &self.things_api_mock
    }
    fn transformations_api(&self) -> &dyn transformations_api::TransformationsApi {
        &self.transformations_api_mock
    }
    fn ui_api(&self) -> &dyn ui_api::UiApi {
        &self.ui_api_mock
    }
    fn uuid_api(&self) -> &dyn uuid_api::UuidApi {
        &self.uuid_api_mock
    }
    fn voice_api(&self) -> &dyn voice_api::VoiceApi {
        &self.voice_api_mock
    }
}

