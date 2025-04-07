use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use clap::Parser;
use hab_rs::{
    event::Event,
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
        mut event_receiver: Receiver<Event>,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        while let Ok(event) = event_receiver.recv().await {
            info!("Got event: {event:?}");
            if let Some(Ok(message)) = event.into_message() {
                info!("Event is message: {message:?}");
            }
            api.items_api().get_item_state1("test_item").await.ok();
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
