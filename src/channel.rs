use crate::event::{ChannelTriggeredEvent, Message, MessageType};

/// Wrapper around an openHAB channel identified by its itentifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel(pub String);

impl Channel {
    /// Channel triggered.
    ///
    /// Checks the entity and optionally the trigger event.
    pub fn received_update<'a>(
        &'_ self,
        message: &'a Message,
        trigger_event: Option<&'_ str>,
    ) -> Option<&'a ChannelTriggeredEvent> {
        match &message.message_type {
            MessageType::ChannelTriggeredEvent(event) if message.topic.entity == self.0 => {
                match trigger_event {
                    Some(trigger_event) if trigger_event == event.event => {
                        return Some(event);
                    }
                    None => return Some(event),
                    _ => (),
                }
            }
            _ => (),
        }
        None
    }
}
