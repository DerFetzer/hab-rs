use std::{error::Error, sync::Arc, time::Duration};

use async_trait::async_trait;
use chrono::{FixedOffset, TimeZone};
use container::OpenhabContainer;
use hab_rs::{
    event::{
        DateTime, Decimal, Event, Hsb, MessageType, OnOff, OpenClosed, Point, Quantity,
        StateUpdatedEvent, TypedValue,
    },
    item::Item,
    rest_api::{Api, ApiClient},
    rest_api_models::{EnrichedItemDto, GroupItemDto},
    rule::{Rule, RuleManager},
};
use palette::Hsv;
use rstest::rstest;
use tokio::time::timeout;
use tracing::info;
use tracing_test::traced_test;

mod container;

struct DummyRule {
    event_tx: tokio::sync::mpsc::Sender<StateUpdatedEvent>,
}

#[async_trait]
impl Rule for DummyRule {
    fn get_name(&self) -> String {
        "DummyRule".to_string()
    }

    async fn run(
        &mut self,
        _api: Arc<dyn Api>,
        mut event_receiver: tokio::sync::broadcast::Receiver<Arc<Event>>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        while let Ok(event) = event_receiver.recv().await {
            info!("Got event: {event:?}");
            if let Event::Message(message) = event.as_ref() {
                if let MessageType::ItemStateUpdatedEvent(updated_event) = &message.message_type {
                    info!("Send updated event: {updated_event:?}");
                    self.event_tx.send(updated_event.clone()).await.unwrap();
                }
            };
        }
        Ok(())
    }
}

struct TestItem {
    name: String,
    item_type: String,
    value: TypedValue,
    api: Arc<ApiClient>,
}

impl TestItem {
    async fn create(&self) -> EnrichedItemDto {
        self.api
            .items_api()
            .add_or_update_item_in_registry(
                &self.name,
                GroupItemDto {
                    name: Some(self.name.clone()),
                    r#type: Some(self.item_type.clone()),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap()
    }

    async fn check(&self) {
        let api_item = self
            .api
            .items_api()
            .get_item_by_name(&self.name, None, None, None)
            .await
            .unwrap();
        info!("Got item: {api_item:#?}");

        let item = Item(self.name.clone());

        let null_state = item.state(self.api.items_api()).await.unwrap();
        assert_eq!(null_state, "NULL");

        item.post_update(self.api.items_api(), &self.value.to_string())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        let value_state = item.state(self.api.items_api()).await.unwrap();
        assert_eq!(value_state, self.value.to_string());
    }
}

#[rstest]
#[case("5.0.0.M2-alpine", 8085)]
#[case("4.3.5-alpine", 8084)]
#[case("4.2.3-alpine", 8083)]
#[case("4.1.3-alpine", 8082)]
#[case("4.0.4-alpine", 8081)]
#[tokio::test]
#[traced_test]
#[ignore]
async fn main(#[case] tag: &str, #[case] host_port: u16) -> Result<(), Box<dyn Error>> {
    let openhab_container = OpenhabContainer::new(tag, host_port).await;
    let config = openhab_container.get_api_configuration(host_port);

    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);

    let mut rule_manager = RuleManager::new(config.clone());
    rule_manager.register(Box::new(DummyRule { event_tx }));

    let api = Arc::new(ApiClient::new(Arc::new(config)));

    let test_items = vec![
        TestItem {
            name: "test_string".to_string(),
            item_type: "String".to_string(),
            value: TypedValue::String("string value".to_string()),
            api: api.clone(),
        },
        TestItem {
            name: "test_switch".to_string(),
            item_type: "Switch".to_string(),
            value: TypedValue::OnOff(OnOff::On),
            api: api.clone(),
        },
        TestItem {
            name: "test_contact".to_string(),
            item_type: "Contact".to_string(),
            value: TypedValue::OpenClosed(OpenClosed::Closed),
            api: api.clone(),
        },
        TestItem {
            name: "test_color".to_string(),
            item_type: "Color".to_string(),
            value: TypedValue::Hsb(Hsb(Hsv::new(180., 0.7, 0.7))),
            api: api.clone(),
        },
        TestItem {
            name: "test_dt".to_string(),
            item_type: "DateTime".to_string(),
            value: TypedValue::DateTime(DateTime(
                FixedOffset::east_opt(0)
                    .unwrap()
                    .with_ymd_and_hms(2025, 5, 4, 11, 42, 15)
                    .unwrap(),
            )),
            api: api.clone(),
        },
        TestItem {
            name: "test_dimmer".to_string(),
            item_type: "Dimmer".to_string(),
            value: TypedValue::Percent(Decimal(0.5)),
            api: api.clone(),
        },
        TestItem {
            name: "test_location_1".to_string(),
            item_type: "Location".to_string(),
            value: TypedValue::Point(Point {
                latitude: 1.1,
                longitude: 2.2,
                altitude: None,
            }),
            api: api.clone(),
        },
        TestItem {
            name: "test_location_2".to_string(),
            item_type: "Location".to_string(),
            value: TypedValue::Point(Point {
                latitude: 1.1,
                longitude: 2.2,
                altitude: Some(3.3),
            }),
            api: api.clone(),
        },
        TestItem {
            name: "test_number".to_string(),
            item_type: "Number".to_string(),
            value: TypedValue::Decimal(Decimal(123.4)),
            api: api.clone(),
        },
        TestItem {
            name: "test_number_dimension".to_string(),
            item_type: "Number:Length".to_string(),
            value: TypedValue::Quantity(Quantity {
                value: 123.4,
                unit: "m".to_string(),
            }),
            api: api.clone(),
        },
    ];

    tokio::spawn(async move {
        info!("Rule manager exited: {:?}", rule_manager.run().await);
    });
    tokio::time::sleep(Duration::from_secs(2)).await;

    for item in test_items {
        let api_item = item.create().await;
        info!("Created item: {api_item:?}");
        info!("Clear queue");
        while let Ok(event) = event_rx.try_recv() {
            info!("Drop event: {event:?}");
        }
        item.check().await;
        info!("Wait for event");
        let updated_event = timeout(Duration::from_secs(2), event_rx.recv())
            .await
            .unwrap()
            .unwrap();
        info!("Got event: {updated_event:?}");
        assert_eq!(updated_event.value, item.value);
    }

    Ok(())
}
