# hab-rs
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

A [Rust][rust] rule engine for [openHAB][openHAB].

Write your home automation rules in Rust and benefit from its vast ecosystem and efficiency.

## Usage

See the `examples\` folder for a full example.

### Quickstart

```rust
use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use hab_rs::{
    event::Event,
    item::Item,
    rest_api::{Api, configuration::Configuration},
    rule::{Rule, RuleManager},
};
use tokio::sync::broadcast::Receiver;


// Define a rule
struct TestRule;

// Implement the Rule trait for your rule struct
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
        while let Ok(event) = event_receiver.recv().await {
            if let Event::Message(message) = event.as_ref() {
                if let Some(command_event) = command_item.received_command(message, None) {
                    println!("Received command event for command_item: {command_event:?}");
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Configuration {
        base_path: "http://your.openhab.instance/rest",
        // openHAB expects the API token as basic auth username with an empty password
        basic_auth: Some(("yourapitoken".to_string(), Some("".to_string()))),
        ..Default::default()
    };

    // Create the rule manager instance
    let mut rule_manager = RuleManager::new(config);

    // Register your rules
    rule_manager.register(Box::new(TestRule));

    // Run the rule manager
    rule_manager.run().await;

    Ok(())
}

```

## License

This program is licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, [DerFetzer][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/DerFetzer
[rust]: https://www.rust-lang.org/
[openHAB]: https://www.openhab.org/

[crates-badge]: https://img.shields.io/crates/v/badge-maker.svg
[crates-url]: https://crates.io/crates/hab-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/DerFetzer/hab-rs/blob/master/LICENSE-MIT
