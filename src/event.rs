use std::{fmt::Display, str::FromStr, sync::LazyLock};

use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::FixedOffset;
use palette::Hsv;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::error::HabRsError;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Event {
    Message(Message),
    Alive,
    Unknown(UnknownEvent),
}

impl FromStr for Event {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.lines().collect::<Vec<_>>().as_slice() {
            [first_line, second_line]
                if first_line.starts_with("event: ") && second_line.starts_with("data: ") =>
            {
                let event_type = first_line
                    .split_once(":")
                    .expect("First line does not contain ':'")
                    .1
                    .trim();
                let data = second_line
                    .split_once(":")
                    .expect("First line does not contain ':'")
                    .1
                    .trim();

                match event_type {
                    "message" => Ok(Self::Message(serde_json::from_str(data)?)),
                    "alive" => Ok(Self::Alive),
                    _ => Ok(Self::Unknown(UnknownEvent {
                        event_type: event_type.to_string(),
                        data: data.to_string(),
                    })),
                }
            }
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownEvent {
    event_type: String,
    data: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Message {
    pub topic: Topic,
    #[serde(flatten)]
    pub message_type: MessageType,
}

impl Message {
    pub fn get_message_type_for_entity(&self, entity: &str) -> Option<&MessageType> {
        if self.topic.entity == entity {
            Some(&self.message_type)
        } else {
            None
        }
    }

    pub fn into_message_type_for_entity(self, entity: &str) -> Option<MessageType> {
        if self.topic.entity == entity {
            Some(self.message_type)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoEvent {
    pub status: String,
    pub status_detail: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[non_exhaustive]
pub struct StateChangedEvent {
    #[serde(flatten)]
    pub value: TypedValue,
    #[serde(flatten)]
    pub old_value: TypedOldValue,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StateUpdatedEvent {
    #[serde(flatten)]
    pub value: TypedValue,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatePredictedEvent {
    #[serde(flatten)]
    pub value: TypedPredictedValue,
    pub is_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ChannelTriggeredEvent {
    pub event: String,
    pub channel: String,
}

macro_rules! typed_values {
    ($([$name:ident, $value_name:literal, $value_type_name:literal]),*) => {
        $(
            #[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
            #[non_exhaustive]
            #[serde(tag = $value_type_name, content = $value_name)]
            pub enum $name {
                Decimal(Decimal),
                Percent(Decimal),
                Quantity(Quantity),
                DateTime(DateTime),
                OnOff(OnOff),
                PlayPause(PlayPause),
                RewindFastforward(RewindFastforward),
                StopMove(StopMove),
                OpenClosed(OpenClosed),
                IncreaseDecrease(IncreaseDecrease),
                UpDown(UpDown),
                NextPrevious(NextPrevious),
                #[serde(rename = "HSB")]
                Hsb(Hsb),
                Point(Point),
                String(String),
                StringList(StringList),
                UnDef(String),
                Raw(Raw),
                Unknown(String),
                #[serde(other)]
                #[default]
                Unimplemented,
            }

            impl Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $name::Decimal(decimal) => decimal.fmt(f),
                        $name::Percent(decimal) => decimal.fmt(f),
                        $name::Quantity(quantity) => quantity.fmt(f),
                        $name::IncreaseDecrease(increase_decrease) => increase_decrease.fmt(f),
                        $name::UpDown(up_down) => up_down.fmt(f),
                        $name::NextPrevious(next_previous) => next_previous.fmt(f),
                        $name::Hsb(color) => color.fmt(f),
                        $name::Point(point) => point.fmt(f),
                        $name::DateTime(date_time) => date_time.fmt(f),
                        $name::OnOff(on_off) => on_off.fmt(f),
                        $name::PlayPause(play_pause) => play_pause.fmt(f),
                        $name::RewindFastforward(rewind_fastforward) => rewind_fastforward.fmt(f),
                        $name::StopMove(stop_move) => stop_move.fmt(f),
                        $name::OpenClosed(open_closed) => open_closed.fmt(f),
                        $name::String(string) => string.fmt(f),
                        $name::StringList(string_list) => string_list.fmt(f),
                        $name::Raw(raw) => raw.fmt(f),
                        $name::UnDef(string) => string.fmt(f),
                        $name::Unknown(string) => string.fmt(f),
                        $name::Unimplemented => write!(f, "Unimplemented"),
                    }
                }
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
                        $name::Percent(decimal) => Self::Percent(decimal),
                        $name::Quantity(quantity) => Self::Quantity(quantity),
                        $name::IncreaseDecrease(increase_decrease) => Self::IncreaseDecrease(increase_decrease),
                        $name::UpDown(up_down) => Self::UpDown(up_down),
                        $name::NextPrevious(next_previous) => Self::NextPrevious(next_previous),
                        $name::Hsb(color) => Self::Hsb(color),
                        $name::Point(point) => Self::Point(point),
                        $name::DateTime(date_time) => Self::DateTime(date_time),
                        $name::OnOff(on_off) => Self::OnOff(on_off),
                        $name::PlayPause(play_pause) => Self::PlayPause(play_pause),
                        $name::RewindFastforward(rewind_fastforward) => Self::RewindFastforward(rewind_fastforward),
                        $name::StopMove(stop_move) => Self::StopMove(stop_move),
                        $name::OpenClosed(open_closed) => Self::OpenClosed(open_closed),
                        $name::String(string) => Self::String(string),
                        $name::StringList(string_list) => Self::StringList(string_list),
                        $name::Raw(raw) => Self::Raw(raw),
                        $name::UnDef(string) => Self::UnDef(string),
                        $name::Unknown(string) => Self::Unknown(string),
                        $name::Unimplemented => Self::Unimplemented,
                    }
                }
            }
        )*
    };
}

from_typed_values!([TypedOldValue, TypedPredictedValue]);

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct Decimal(pub f64);

impl FromStr for Decimal {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(f64::from_str(s)?))
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
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

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

impl FromStr for Point {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").collect::<Vec<_>>().as_slice() {
            [latitude, longitude] => Ok(Self {
                latitude: f64::from_str(latitude)?,
                longitude: f64::from_str(longitude)?,
                altitude: None,
            }),
            [latitude, longitude, altitude] => Ok(Self {
                latitude: f64::from_str(latitude)?,
                longitude: f64::from_str(longitude)?,
                altitude: Some(f64::from_str(altitude)?),
            }),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.latitude, self.longitude)?;
        if let Some(altitude) = &self.altitude {
            write!(f, ",{}", altitude)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct Raw {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl FromStr for Raw {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(";").collect::<Vec<_>>().as_slice() {
            [mime_type, data] if mime_type.starts_with("data:") && data.starts_with("base64,") => {
                Ok(Self {
                    mime_type: mime_type
                        .split_once(":")
                        .ok_or_else(|| HabRsError::Parse(s.to_string()))?
                        .1
                        .to_string(),
                    data: BASE64_STANDARD.decode(
                        data.split_once(",")
                            .ok_or_else(|| HabRsError::Parse(s.to_string()))?
                            .1,
                    )?,
                })
            }
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for Raw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "data:{};base64,{}",
            self.mime_type,
            BASE64_STANDARD.encode(&self.data)
        )
    }
}

static DELIMITER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^\\],").expect("Invalid regex"));

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct StringList(Vec<String>);

impl FromStr for StringList {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let delim_matches: Vec<_> = DELIMITER_RE.find_iter(s).map(|m| m.start() + 1).collect();
        let mut strings = Vec::with_capacity(delim_matches.len() + 1);

        for i in 0..=delim_matches.len() {
            let start = if i == 0 { 0 } else { delim_matches[i - 1] + 1 };
            let end = if i == delim_matches.len() {
                s.len()
            } else {
                delim_matches[i]
            };
            strings.push(s[start..end].replace("\\,", ","));
        }

        Ok(Self(strings))
    }
}

impl Display for StringList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| s.replace(",", "\\,"))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum IncreaseDecrease {
    #[default]
    Increase,
    Decrease,
}

impl FromStr for IncreaseDecrease {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INCREASE" => Ok(Self::Increase),
            "DECREASE" => Ok(Self::Decrease),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for IncreaseDecrease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increase => write!(f, "INCREASE"),
            Self::Decrease => write!(f, "DECREASE"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum NextPrevious {
    #[default]
    Next,
    Previous,
}

impl FromStr for NextPrevious {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEXT" => Ok(Self::Next),
            "PREVIOUS" => Ok(Self::Previous),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for NextPrevious {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Next => write!(f, "NEXT"),
            Self::Previous => write!(f, "PREVIOUS"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum PlayPause {
    #[default]
    Play,
    Pause,
}

impl FromStr for PlayPause {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLAY" => Ok(Self::Play),
            "PAUSE" => Ok(Self::Pause),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for PlayPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Play => write!(f, "PLAY"),
            Self::Pause => write!(f, "PAUSE"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum RewindFastforward {
    #[default]
    Rewind,
    Fastforward,
}

impl FromStr for RewindFastforward {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REWIND" => Ok(Self::Rewind),
            "FASTFORWARD" => Ok(Self::Fastforward),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for RewindFastforward {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rewind => write!(f, "REWIND"),
            Self::Fastforward => write!(f, "FASTFORWARD"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum StopMove {
    #[default]
    Stop,
    Move,
}

impl FromStr for StopMove {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STOP" => Ok(Self::Stop),
            "MOVE" => Ok(Self::Move),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for StopMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stop => write!(f, "STOP"),
            Self::Move => write!(f, "MOVE"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum UpDown {
    #[default]
    Up,
    Down,
}

impl FromStr for UpDown {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UP" => Ok(Self::Up),
            "DOWN" => Ok(Self::Down),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for UpDown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "UP"),
            Self::Down => write!(f, "DOWN"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct Hsb(pub Hsv);

impl FromStr for Hsb {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").collect::<Vec<_>>().as_slice() {
            [h, s, b] => Ok(Self(Hsv::new_srgb(
                f32::from_str(h)?,
                f32::from_str(s)? / 100.0,
                f32::from_str(b)? / 100.0,
            ))),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for Hsb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.0.hue.into_positive_degrees(),
            self.0.saturation * 100.0,
            self.0.value * 100.0
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub struct DateTime(pub chrono::DateTime<FixedOffset>);

impl FromStr for DateTime {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(chrono::DateTime::from_str(s)?))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%.3f%z"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum OnOff {
    #[default]
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

impl Display for OnOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "ON"),
            Self::Off => write!(f, "OFF"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
pub enum OpenClosed {
    #[default]
    Open,
    Closed,
}

impl FromStr for OpenClosed {
    type Err = HabRsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPEN" => Ok(Self::Open),
            "CLOSED" => Ok(Self::Closed),
            _ => Err(HabRsError::Parse(s.to_string())),
        }
    }
}

impl Display for OpenClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, DeserializeFromStr, SerializeDisplay, Default)]
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

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.namespace, self.entity_type, self.entity, self.action
        )
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_parse_event() {
        let event_str = r#"event: message
data: {"topic":"openhab/things/jeelink:lacrosse:40/status","payload":"{\"status\":\"ONLINE\",\"statusDetail\":\"NONE\"}","type":"ThingStatusInfoEvent"}"#;

        let event = event_str.parse::<Event>().unwrap();
        assert_eq!(
            event,
            Event::Message(Message {
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
            })
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

    #[test]
    fn test_string_list() {
        let s = r"FirstString,Second\,String\,,ThirdString";
        let string_list = StringList::from_str(s).unwrap();
        assert_eq!(
            string_list,
            StringList(vec![
                "FirstString".to_string(),
                "Second,String,".to_string(),
                "ThirdString".to_string()
            ])
        );
    }

    macro_rules! enum_test {
        ($first:ident, $second:ident) => {
            paste! {
                #[rstest]
                #[case::[<value _ $first:lower>]([<$first $second>]::$first, stringify!([<$first:upper>]))]
                #[case::[<value _ $second:lower>]([<$first $second>]::$second, stringify!([<$second:upper>]))]
                fn [<test_ $first:lower _ $second:lower>](#[case] val: [<$first $second>], #[case] exp_str: &str) {
                    let str = val.to_string();
                    assert_eq!(str, exp_str);
                    let val_from_str: [<$first $second>] = str.parse().unwrap();
                    assert_eq!(val_from_str, val);
                }
            }
        };
    }

    enum_test!(Increase, Decrease);
    enum_test!(Up, Down);
    enum_test!(Next, Previous);
    enum_test!(On, Off);
    enum_test!(Play, Pause);
    enum_test!(Rewind, Fastforward);
    enum_test!(Stop, Move);
    enum_test!(Open, Closed);
}
