#[cfg(feature = "items_api")]
use hab_rs_api_client::apis::items_api::{
    GetItemState1Error, ItemsApi, SendItemCommandError, UpdateItemStateError,
};
use tracing::{error, instrument};

use crate::event::{Message, MessageType, StateChangedEvent, StateUpdatedEvent};

/// Wrapper around an openHAB item identified by its name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item(pub String);

#[cfg(feature = "items_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "items_api")))]
impl Item {
    /// Send a command to an item.
    ///
    /// See <https://www.openhab.org/docs/configuration/rules-dsl.html#manipulating-item-states>.
    #[instrument(skip(items_api))]
    pub async fn send_command(
        &self,
        items_api: &dyn ItemsApi,
        command: &str,
    ) -> Result<(), hab_rs_api_client::apis::Error<SendItemCommandError>> {
        items_api.send_item_command(&self.0, command).await
    }

    /// Post an update for an item.
    ///
    /// See <https://www.openhab.org/docs/configuration/rules-dsl.html#manipulating-item-states>.
    #[instrument(skip(items_api))]
    pub async fn post_update(
        &self,
        items_api: &dyn ItemsApi,
        command: &str,
    ) -> Result<(), hab_rs_api_client::apis::Error<UpdateItemStateError>> {
        items_api.update_item_state(&self.0, command, None).await
    }

    /// Get the state of an item.
    ///
    /// See <https://www.openhab.org/docs/configuration/rules-dsl.html#using-the-states-of-items-in-rules>.
    #[instrument(skip(items_api))]
    pub async fn state(
        &self,
        items_api: &dyn ItemsApi,
    ) -> Result<String, hab_rs_api_client::apis::Error<GetItemState1Error>> {
        items_api.get_item_state1(&self.0).await
    }
}

impl Item {
    /// Item received a command.
    ///
    /// Checks the entity and optionally the received command.
    pub fn received_command<'a>(
        &'_ self,
        message: &'a Message,
        command: Option<&'_ str>,
    ) -> Option<&'a StateUpdatedEvent> {
        match &message.message_type {
            MessageType::ItemCommandEvent(event) if message.topic.entity == self.0 => match command
            {
                Some(command) if command == event.value.to_string() => {
                    return Some(event);
                }
                None => return Some(event),
                _ => (),
            },
            _ => (),
        }
        None
    }

    /// Item received an update.
    ///
    /// Checks the entity and optionally the state.
    pub fn received_update<'a>(
        &'_ self,
        message: &'a Message,
        state: Option<&'_ str>,
    ) -> Option<&'a StateUpdatedEvent> {
        match &message.message_type {
            MessageType::ItemStateUpdatedEvent(event) if message.topic.entity == self.0 => {
                match state {
                    Some(command) if command == event.value.to_string() => {
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

    /// Item changed its value.
    ///
    /// Checks the entity and optionally the old and/or new state.
    pub fn changed<'a>(
        &'_ self,
        message: &'a Message,
        from: Option<&'_ str>,
        to: Option<&'_ str>,
    ) -> Option<&'a StateChangedEvent> {
        match &message.message_type {
            MessageType::ItemStateChangedEvent(event) if message.topic.entity == self.0 => {
                match (from, to) {
                    (Some(from), Some(to))
                        if from == event.old_value.to_string() && to == event.value.to_string() =>
                    {
                        return Some(event);
                    }
                    (Some(from), None) if from == event.old_value.to_string() => {
                        return Some(event);
                    }
                    (None, Some(to)) if to == event.value.to_string() => {
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

    /// Member of a group changed its value.
    ///
    /// Checks the entity and optionally the old and/or new state.
    /// Returns the triggering item name as well.
    pub fn member_of_group_changed<'a>(
        &'_ self,
        message: &'a Message,
        from: Option<&'_ str>,
        to: Option<&'_ str>,
    ) -> Option<(&'a str, &'a StateChangedEvent)> {
        match &message.message_type {
            MessageType::GroupItemStateChangedEvent(event) if message.topic.entity == self.0 => {
                if let Some(sub_entity) = &message.topic.sub_entity {
                    match (from, to) {
                        (Some(from), Some(to))
                            if from == event.old_value.to_string()
                                && to == event.value.to_string() =>
                        {
                            return Some((sub_entity, event));
                        }
                        (Some(from), None) if from == event.old_value.to_string() => {
                            return Some((sub_entity, event));
                        }
                        (None, Some(to)) if to == event.value.to_string() => {
                            return Some((sub_entity, event));
                        }
                        (None, None) => return Some((sub_entity, event)),
                        _ => (),
                    }
                } else {
                    error!(
                        "GroupItemStateChangedEvent topic has no sub_entity. This is probably a bug in OpenHAB!"
                    );
                }
            }
            _ => (),
        }
        None
    }
}
