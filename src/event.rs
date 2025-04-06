use std::str::FromStr;

use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;
use serde_with::DeserializeFromStr;

use crate::error::HabRsError;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub event_type: EventType,
    pub data: Value,
}

impl Event {
    pub fn into_message(self) -> Option<Result<Message, HabRsError>> {
        if self.event_type != EventType::Message {
            None
        } else {
            Some(serde_json::from_value(self.data).map_err(|e| e.into()))
        }
    }
}

impl FromStr for Event {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.lines().collect::<Vec<_>>().as_slice() {
            [first_line, second_line]
                if first_line.starts_with("event: ") && second_line.starts_with("data: ") =>
            {
                Ok(Self {
                    event_type: first_line.trim_start_matches("event: ").parse()?,
                    data: serde_json::from_str(second_line.trim_start_matches("data: "))?,
                })
            }
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum EventType {
    Message,
    Alive,
    Unknown(String),
}

impl FromStr for EventType {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "message" => Ok(Self::Message),
            "alive" => Ok(Self::Alive),
            s if !s.is_empty() && s.is_ascii() && s.chars().all(|c| !c.is_whitespace()) => {
                Ok(Self::Unknown(s.to_string()))
            }
            s => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Message {
    pub topic: Topic,
    #[serde(flatten)]
    pub message_type: MessageType,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", content = "payload")]
pub enum MessageType {
    #[serde(with = "serde_nested_json")]
    ItemStateEvent(StateChangedEvent),
    #[serde(with = "serde_nested_json")]
    ItemStateChangedEvent(StateChangedEvent),
    #[serde(with = "serde_nested_json")]
    GroupItemStateChangedEvent(StateChangedEvent),
    #[serde(with = "serde_nested_json")]
    ItemStateUpdatedEvent(StateUpdatedEvent),
    #[serde(with = "serde_nested_json")]
    ItemStatePredictedEvent(StatePredictedEvent),
    #[serde(with = "serde_nested_json")]
    GroupStateUpdatedEvent(StateUpdatedEvent),
    #[serde(with = "serde_nested_json")]
    ItemCommandEvent(StateUpdatedEvent),
    #[serde(with = "serde_nested_json")]
    RuleStatusInfoEvent(StatusInfoEvent),
    #[serde(with = "serde_nested_json")]
    ThingStatusInfoEvent(StatusInfoEvent),
    #[serde(with = "serde_nested_json")]
    ThingStatusInfoChangedEvent([StatusInfoEvent; 2]),
    #[serde(with = "serde_nested_json")]
    ChannelTriggeredEvent(ChannelTriggeredEvent),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoEvent {
    pub status: String,
    pub status_detail: String,
    pub description: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StateChangedEvent {
    #[serde(flatten)]
    pub value: TypedValue,
    #[serde(flatten)]
    pub old_value: TypedOldValue,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StateUpdatedEvent {
    #[serde(flatten)]
    pub value: TypedValue,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatePredictedEvent {
    #[serde(flatten)]
    pub value: TypedPredictedValue,
    pub is_confirmation: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ChannelTriggeredEvent {
    pub event: String,
    pub channel: String,
}

macro_rules! typed_values {
    ($([$name:ident, $value_name:literal, $value_type_name:literal]),*) => {
        $(
            #[derive(Debug, PartialEq, Deserialize)]
            #[non_exhaustive]
            #[serde(tag = $value_type_name, content = $value_name)]
            pub enum $name {
                Decimal(Decimal),
                Quantity(Quantity),
                DateTime(DateTime),
                OnOff(OnOff),
                String(String),
                UnDef(String),
                Unknown(String),
            }
        )*
    };
}

typed_values!(
    [TypedValue, "value", "type"],
    [TypedOldValue, "oldValue", "oldType"],
    [TypedPredictedValue, "predictedValue", "predictedType"]
);

macro_rules! from_typed_values {
    ([$($name:ident),*]) => {
        $(
            impl From<$name> for TypedValue {
                fn from(value: $name) -> Self {
                    match value {
                        $name::Decimal(decimal) => Self::Decimal(decimal),
                        $name::Quantity(quantity) => Self::Quantity(quantity),
                        $name::DateTime(date_time) => Self::DateTime(date_time),
                        $name::OnOff(on_off) => Self::OnOff(on_off),
                        $name::String(string) => Self::String(string),
                        $name::UnDef(string) => Self::UnDef(string),
                        $name::Unknown(string) => Self::Unknown(string),
                    }
                }
            }
        )*
    };
}

from_typed_values!([TypedOldValue, TypedPredictedValue]);

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub struct Decimal(pub f64);

impl FromStr for Decimal {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(f64::from_str(s)?))
    }
}

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
}

impl FromStr for Quantity {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<_>>().as_slice() {
            [value, unit] => Ok(Self {
                value: f64::from_str(value)?,
                unit: unit.to_string(),
            }),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub struct DateTime(pub chrono::DateTime<Utc>);

impl FromStr for DateTime {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(chrono::DateTime::from_str(s)?))
    }
}

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub enum OnOff {
    On,
    Off,
}

impl FromStr for OnOff {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ON" => Ok(Self::On),
            "OFF" => Ok(Self::Off),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub struct Topic {
    pub namespace: String,
    pub entity_type: String,
    pub entity: String,
    pub action: String,
}

impl FromStr for Topic {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| c.is_whitespace()) {
            return Err(HabRsError::Parse(s.to_string()));
        }
        match s.split("/").collect::<Vec<_>>().as_slice() {
            [namespace, entity_type, entity, action] => Ok(Self {
                namespace: namespace.to_string(),
                entity_type: entity_type.to_string(),
                entity: entity.to_string(),
                action: action.to_string(),
            }),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_parse_event() {
        let event_str = r#"event: message
data: {"topic":"openhab/things/jeelink:lacrosse:40/status","payload":"{\"status\":\"ONLINE\",\"statusDetail\":\"NONE\"}","type":"ThingStatusInfoEvent"}"#;

        let event = event_str.parse::<Event>().unwrap();
        assert_eq!(
            event,
            Event {
                event_type: EventType::Message,
                data: json!({"topic":"openhab/things/jeelink:lacrosse:40/status","payload":"{\"status\":\"ONLINE\",\"statusDetail\":\"NONE\"}","type":"ThingStatusInfoEvent"})
            }
        );

        let message = event.into_message().unwrap().unwrap();
        assert_eq!(
            message,
            Message {
                topic: Topic {
                    namespace: "openhab".to_string(),
                    entity_type: "things".to_string(),
                    entity: "jeelink:lacrosse:40".to_string(),
                    action: "status".to_string(),
                },
                message_type: MessageType::ThingStatusInfoEvent(StatusInfoEvent {
                    status: "ONLINE".to_string(),
                    status_detail: "NONE".to_string(),
                    description: None,
                }),
            }
        );
    }

    #[test]
    fn test_state_changed_event() {
        let message_data = r#"{"topic":"openhab/items/Arbeit_Steck_P_power/statechanged","payload":"{\"type\":\"Decimal\",\"value\":\"222.23\",\"oldType\":\"Decimal\",\"oldValue\":\"225.99\"}","type":"ItemStateChangedEvent"}"#;
        let message: Message = serde_json::from_str(message_data).unwrap();
        assert_eq!(
            message.message_type,
            MessageType::ItemStateChangedEvent(StateChangedEvent {
                value: TypedValue::Decimal(Decimal(222.23)),
                old_value: TypedOldValue::Decimal(Decimal(225.99))
            })
        );
    }
}
