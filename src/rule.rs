use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use async_trait::async_trait;
use futures_util::StreamExt;
use hab_rs_api_client::apis::{Api, ApiClient, configuration::Configuration};
use reqwest::Client;
use tokio::{
    sync::broadcast::{self, Receiver},
    task::{Id, JoinSet},
};
use tracing::{debug, error, info, warn};

use crate::event::Event;

pub struct RuleManager {
    api: Arc<ApiClient>,
    config: Configuration,
    rules: Vec<Box<dyn Rule>>,
}

impl RuleManager {
    pub fn new(config: Configuration) -> Self {
        RuleManager {
            api: Arc::new(ApiClient::new(Arc::new(config.clone()))),
            config,
            rules: vec![],
        }
    }

    pub fn register(&mut self, rule: Box<dyn Rule>) {
        let rule_name = rule.get_name();
        info!("Register rule {rule_name}");
        self.rules.push(rule);
    }

    pub async fn run(self) {
        let (event_tx, _event_rx) = broadcast::channel(100);
        let mut rules_set = JoinSet::new();
        let mut rule_task_names = HashMap::new();

        for mut rule in self.rules {
            let event_tx = event_tx.subscribe();
            let api = self.api.clone();
            let rule_name = rule.get_name();
            info!("Start rule {rule_name}");
            let rule_id = rules_set
                .spawn(async move { rule.run(api, event_tx).await })
                .id();
            info!("Started rule {rule_name} with id={rule_id}");
            rule_task_names.insert(rule_id, rule_name);
        }

        let config = self.config.clone();
        tokio::spawn(async move {
            let client = Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .build()
                .expect("Invalid client configuration");
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let res: Result<(), Box<dyn Error>> = async {
                    info!("Started event receiver");
                    let mut request_builder = client.get(format!("{}/events", config.base_path));
                    if let Some((username, password)) = &config.basic_auth {
                        request_builder = request_builder.basic_auth(username, password.clone());
                    }
                    let mut stream = request_builder.send().await?.bytes_stream();

                    let mut buf = vec![];

                    while let Some(chunk) = stream.next().await {
                        buf.extend(chunk?.into_iter());
                        // Check for double line break
                        if buf.ends_with(&[0x0A, 0x0A]) {
                            let event_string = String::from_utf8(buf.clone())?;
                            buf.clear();

                            match event_string.trim().parse() {
                                Ok(event) => {
                                    debug!("Got event from stream: {event:?}");
                                    event_tx.send(event).ok();
                                }
                                Err(e) => {
                                    warn!("Could not parse event: {e:?}");
                                }
                            }
                        }
                    }
                    Ok(())
                }
                .await;
                match res {
                    Ok(_) => warn!("Event task exited without error"),
                    Err(e) => error!("Event task exited with error: {e:?}"),
                };
            }
        });

        while let Some(res) = rules_set.join_next_with_id().await {
            match res {
                Ok((id, Ok(_))) => warn!(
                    "Rule {} exited.",
                    Self::get_name_from_id(id, &rule_task_names)
                ),
                Ok((id, Err(e))) => error!(
                    "Rule {} exited with error: {e}",
                    Self::get_name_from_id(id, &rule_task_names)
                ),
                Err(e) if e.is_panic() => error!(
                    "Rule {} panicked",
                    Self::get_name_from_id(e.id(), &rule_task_names)
                ),
                Err(e) if e.is_cancelled() => warn!(
                    "Rule {} was cancelled",
                    Self::get_name_from_id(e.id(), &rule_task_names)
                ),
                _ => error!("Could not join rule"),
            }
        }
        error!("All rules exited");
    }

    fn get_name_from_id(id: Id, rule_task_names: &HashMap<Id, String>) -> &str {
        rule_task_names
            .get(&id)
            .expect("Could not find name for id")
    }
}

#[async_trait]
pub trait Rule: Send {
    fn get_name(&self) -> String;

    async fn run(
        &mut self,
        api: Arc<dyn Api>,
        event_receiver: Receiver<Event>,
    ) -> Result<(), Box<dyn std::error::Error + Send>>;
}

#[cfg(test)]
mod tests {
    use hab_rs_api_client::apis::{MockApiClient, configuration::Configuration};
    use tracing_test::traced_test;

    use super::*;

    struct TestRule;

    #[async_trait]
    impl Rule for TestRule {
        fn get_name(&self) -> String {
            "TestRule".to_string()
        }

        async fn run(
            &mut self,
            api: Arc<dyn Api>,
            _event_receiver: Receiver<Event>,
        ) -> Result<(), Box<dyn std::error::Error + Send>> {
            api.items_api().get_item_state1("test_item").await.ok();
            Ok(())
        }
    }

    #[traced_test]
    #[tokio::test]
    async fn test_manager() {
        let mut rule_manager = RuleManager::new(Configuration::new());
        rule_manager.register(Box::new(TestRule));
        rule_manager.run().await;
    }

    #[tokio::test]
    async fn test_rule_mock() {
        let (_event_tx, event_rx) = broadcast::channel(1);
        let mut api = MockApiClient::new();
        api.items_api_mock
            .expect_get_item_state1()
            .with(mockall::predicate::eq("test_item"))
            .times(1)
            .returning(|_| Ok(42.to_string()));

        TestRule.run(Arc::new(api), event_rx).await.unwrap();
    }
}
