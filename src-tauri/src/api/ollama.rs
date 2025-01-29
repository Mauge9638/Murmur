use std::time::Duration;

use ureq::Agent;

pub fn get_api_tags() -> String {
    let mut config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();

    let agent: Agent = config.into();

    let response = agent
        .get("http://localhost:11434/api/tags")
        .call()
        .expect("something went wrong with the request")
        .body_mut()
        .read_to_string()
        .expect("Test");
    return response;
}
