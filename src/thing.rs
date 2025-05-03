use crate::event::{Message, MessageType, StatusInfoEvent};

/// Wrapper around an openHAB thing identified by its name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Thing(pub String);

impl Thing {
    /// Thing received an update.
    ///
    /// Checks the entity and optionally the status.
    pub fn received_update<'a>(
        &'_ self,
        message: &'a Message,
        state: Option<&'_ str>,
    ) -> Option<&'a StatusInfoEvent> {
        match &message.message_type {
            MessageType::ThingStatusInfoEvent(event) if message.topic.entity == self.0 => {
                match state {
                    Some(command) if command == event.status => {
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

    /// Thing changed its status.
    ///
    /// Checks the entity and optionally the old and/or new status.
    pub fn changed<'a>(
        &'_ self,
        message: &'a Message,
        from: Option<&'_ str>,
        to: Option<&'_ str>,
        ignore_change_in_status_detail: bool,
    ) -> Option<&'a [StatusInfoEvent; 2]> {
        match &message.message_type {
            MessageType::ThingStatusInfoChangedEvent(event) if message.topic.entity == self.0 => {
                let new_status = &event[0];
                let old_status = &event[1];
                if ignore_change_in_status_detail && new_status.status == old_status.status {
                    // Only status detail changed
                    return None;
                }
                match (from, to) {
                    (Some(from), Some(to))
                        if from == old_status.status && to == new_status.status =>
                    {
                        return Some(event);
                    }
                    (Some(from), None) if from == old_status.status => {
                        return Some(event);
                    }
                    (None, Some(to)) if to == new_status.status => {
                        return Some(event);
                    }
                    (None, None) => return Some(event),
                    _ => (),
                }
            }
            _ => (),
        }
        None
    }
}
