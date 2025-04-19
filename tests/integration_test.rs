use std::{error::Error, sync::Arc};

use container::OpenhabContainer;
use hab_rs::{
    item::Item,
    rest_api::{Api, ApiClient},
    rest_api_models::GroupItemDto,
};

mod container;

#[tokio::test]
#[ignore]
async fn main() -> Result<(), Box<dyn Error>> {
    let openhab_container = OpenhabContainer::new().await;
    let config = openhab_container.get_api_configuration();
    let api = ApiClient::new(Arc::new(config));
    let items_api = api.items_api();
    let created_item = items_api
        .add_or_update_item_in_registry(
            "test_string",
            GroupItemDto {
                name: Some("test_string".to_string()),
                r#type: Some("String".to_string()),
                ..Default::default()
            },
            None,
        )
        .await?;
    println!("Created item: {created_item:#?}");
    let item = items_api
        .get_item_by_name("test_string", None, None, None)
        .await?;
    println!("Got item: {item:#?}");

    let string_item = Item("test_string".to_string());

    let null_state = string_item.state(items_api).await?;
    assert_eq!(null_state, "NULL");

    string_item.post_update(items_api, "Init Value").await?;
    let init_state = string_item.state(items_api).await?;
    assert_eq!(init_state, "Init Value");

    Ok(())
}
