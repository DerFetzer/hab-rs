use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;
use serde_nested_json;
use serde_with::DeserializeFromStr;

use crate::error::HabRsError;

#[derive(Debug, PartialEq)]
pub struct Event {
    event_type: EventType,
    data: Value,
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

#[derive(Debug, PartialEq)]
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
    topic: Topic,
    #[serde(with = "serde_nested_json")]
    payload: Value,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, PartialEq, DeserializeFromStr)]
pub struct Topic {
    namespace: String,
    entity_type: String,
    entity: String,
    action: String,
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
                payload: json!({"status":"ONLINE","statusDetail":"NONE"}),
                message_type: "ThingStatusInfoEvent".to_string(),
            }
        );
    }
}
