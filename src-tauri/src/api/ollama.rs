use std::io::BufRead;
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
    response
}

pub fn post_generate<F>(message: &str, mut callback: F)
where
    F: FnMut(String),
{
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();

    let agent: Agent = config.into();

    let json_body = serde_json::json!({
        "model": "deepseek-r1:14b",
        "prompt": message,
        "stream": true
    });

    match agent
        .post("http://localhost:11434/api/generate")
        .header("Content-Type", "application/json")
        .send_json(&json_body)
    {
        Ok(response) => {
            let (_, body) = response.into_parts();
            let reader = std::io::BufReader::new(body.into_reader());

            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(chunk) = serde_json::from_str::<serde_json::Value>(&line) {
                        if let Some(response) = chunk["response"].as_str() {
                            callback(response.to_string());
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Request failed: {}", e),
    }
}
