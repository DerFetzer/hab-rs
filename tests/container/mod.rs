use std::time::Duration;

use hab_rs::rest_api::configuration::Configuration;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

pub struct OpenhabContainer {
    _container: ContainerAsync<GenericImage>,
    api_token: String,
}

impl OpenhabContainer {
    pub async fn new(tag: &str, host_port: u16) -> OpenhabContainer {
        let container = GenericImage::new("openhab/openhab", tag)
            .with_wait_for(WaitFor::Duration {
                length: Duration::from_secs(20),
            })
            .with_mapped_port(host_port, 8080.tcp())
            .with_startup_timeout(Duration::from_secs(120))
            .start()
            .await
            .expect("Could not start openhab container");
        println!("Container started");

        // create admin user and API token
        let mut exec_result = container
        .exec(ExecCommand::new(["sh", "-c", "echo \"openhab:users add admin admin administrator;openhab:users addApiToken admin token authentication\" | /openhab/runtime/bin/client -u openhab -p habopen -b"]))
        .await
        .expect("Could not prepare container");
        println!("Executed command");

        let stdout = String::from_utf8(
            exec_result
                .stdout_to_vec()
                .await
                .expect("Could not get command output"),
        )
        .unwrap();
        let stderr = String::from_utf8(
            exec_result
                .stderr_to_vec()
                .await
                .expect("Could not get command output"),
        )
        .unwrap();
        println!("stdout: {stdout}");
        println!("stderr: {stderr}");

        let api_token = stdout
            .lines()
            .last()
            .expect("No command output")
            .to_string();
        OpenhabContainer {
            _container: container,
            api_token,
        }
    }

    pub fn get_api_configuration(&self, host_port: u16) -> Configuration {
        Configuration {
            base_path: format!("http://localhost:{host_port}/rest"),
            basic_auth: Some((self.api_token.clone(), Some(String::new()))),
            ..Configuration::default()
        }
    }
}
