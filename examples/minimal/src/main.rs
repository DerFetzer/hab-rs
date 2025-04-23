use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use clap::Parser;
use hab_rs::{
    event::Event,
    item::Item,
    rest_api::{Api, configuration::Configuration},
    rule::{Rule, RuleManager},
};
use tokio::sync::broadcast::Receiver;
use tracing::info;

#[derive(Debug, Parser)]
struct Cli {
    rest_url: String,
    api_token: String,
}

struct TestRule;

#[async_trait]
impl Rule for TestRule {
    fn get_name(&self) -> String {
        "TestRule".to_string()
    }

    async fn run(
        &mut self,
        api: Arc<dyn Api>,
        mut event_receiver: Receiver<Arc<Event>>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        let command_item = Item("command_item".to_string());
        let test_item = Item("test_item".to_string());
        while let Ok(event) = event_receiver.recv().await {
            info!("Got event: {event:?}");
            if let Event::Message(message) = event.as_ref() {
                info!("Event is message: {message:?}");
                if let Some(command_event) = command_item.received_command(message, None) {
                    info!("Received command event for command_item: {command_event:?}");
                }
            }
            if let Ok(state) = test_item.state(api.items_api()).await {
                info!("Item state: {state}");
            }
        }
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    let config = Configuration {
        base_path: cli.rest_url,
        basic_auth: Some((cli.api_token, Some("".to_string()))),
        ..Default::default()
    };

    let mut rule_manager = RuleManager::new(config);
    rule_manager.register(Box::new(TestRule));
    rule_manager.run().await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use hab_rs::{
        event::{Message, MessageType, OnOff, StateUpdatedEvent, Topic, TypedValue},
        rest_api::MockApiClient,
    };
    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    use super::*;

    #[tokio::test(start_paused = true)]
    #[traced_test]
    async fn test_rule() {
        let (event_tx, event_rx) = broadcast::channel(1);

        // Create mock and set expectation
        let mut api = MockApiClient::new();
        api.items_api_mock
            .expect_get_item_state1()
            .with(mockall::predicate::eq("test_item"))
            .times(1)
            .returning(|_| Ok("ON".to_string()));

        let api = Arc::new(api);
        let task_api = api.clone();

        // Spawn task that runs the rule
        let run_handle = tokio::spawn(async move {
            let mut rule = TestRule;
            rule.run(task_api, event_rx).await
        });

        // Send event
        let mut state_updated_event = StateUpdatedEvent::default();
        state_updated_event.value = TypedValue::OnOff(OnOff::On);
        let message = Message {
            topic: Topic {
                namespace: String::new(),
                entity_type: String::new(),
                entity: "command_item".to_string(),
                sub_entity: None,
                action: String::new(),
            },
            message_type: MessageType::ItemCommandEvent(state_updated_event),
        };
        let on_event = Arc::new(Event::Message(message));
        event_tx.send(on_event).unwrap();

        tokio::time::advance(Duration::from_millis(500)).await;

        // Check that task is still running
        assert!(!run_handle.is_finished());

        run_handle.abort();

        // Yield so that the single threaded runtime is able to abort the task
        tokio::task::yield_now().await;
        while !run_handle.is_finished() {}

        // Make sure the mocks are dropped inside the test function so that errors are detected.
        while Arc::strong_count(&api) > 1 {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
