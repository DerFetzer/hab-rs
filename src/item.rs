use hab_rs_api_client::apis::items_api::GetItemState1Error;
#[cfg(feature = "items_api")]
use hab_rs_api_client::apis::items_api::{ItemsApi, SendItemCommandError, UpdateItemStateError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item(pub String);

#[cfg(feature = "items_api")]
impl Item {
    pub async fn send_command(
        &self,
        items_api: &dyn ItemsApi,
        command: &str,
    ) -> Result<(), hab_rs_api_client::apis::Error<SendItemCommandError>> {
        items_api.send_item_command(&self.0, command).await
    }

    pub async fn post_update(
        &self,
        items_api: &dyn ItemsApi,
        command: &str,
    ) -> Result<(), hab_rs_api_client::apis::Error<UpdateItemStateError>> {
        items_api.update_item_state(&self.0, command, None).await
    }

    pub async fn state(
        &self,
        items_api: &dyn ItemsApi,
    ) -> Result<String, hab_rs_api_client::apis::Error<GetItemState1Error>> {
        items_api.get_item_state1(&self.0).await
    }
}
