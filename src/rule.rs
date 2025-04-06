use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use hab_rs_api_client::apis::Api;
use tokio::{
    sync::broadcast::{self, Receiver},
    task::{Id, JoinSet},
};
use tracing::{error, info, warn};

use crate::event::Event;

pub struct RuleManager<A: Api> {
    api: Arc<A>,
    rules: Vec<Box<dyn Rule>>,
}

impl<A: Api + 'static> RuleManager<A> {
    pub fn new(api: A) -> Self {
        RuleManager {
            api: Arc::new(api),
            rules: vec![],
        }
    }

    pub fn register(&mut self, rule: Box<dyn Rule>) {
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
    use hab_rs_api_client::apis::{ApiClient, MockApiClient, configuration::Configuration};

    use super::*;

    struct TestRule;

    #[async_trait]
    impl Rule for TestRule {
        fn get_name(&self) -> String {
            "TestRule".to_string()
        }

        async fn run(
            &mut self,
            _api: Arc<dyn Api>,
            _event_receiver: Receiver<Event>,
        ) -> Result<(), Box<dyn std::error::Error + Send>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_manager() {
        let mut rule_manager = RuleManager::new(ApiClient::new(Arc::new(Configuration::new())));
        rule_manager.register(Box::new(TestRule));
        rule_manager.run().await;
    }

    #[tokio::test]
    async fn test_manager_mock() {
        let mut rule_manager = RuleManager::new(MockApiClient::new());
        rule_manager.register(Box::new(TestRule));
        rule_manager.run().await;
    }
}
