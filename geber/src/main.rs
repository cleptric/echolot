use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::{fs, time::Duration};

#[derive(Deserialize, Clone)]
struct Config {
    upstream: String,
    name: String,
    api_token: String,
}

#[derive(Deserialize, Clone)]
struct Monitor {
    slug: String,
    url: String,
    frequency: u64,
}

#[tokio::main]
async fn main() {
    let file = fs::read_to_string("config/config.yaml").expect("Can not open config file!");
    let config: Config = serde_yaml::from_str(&file).unwrap();

    let file = fs::read_to_string("config/monitors.json").expect("Can not open monitors config file!");
    let monitors: Vec<Monitor> = serde_json::from_str(&file).unwrap();

    for monitor in monitors {
        let config = config.clone();

        tokio::spawn(async {
            perfom_uptime_check(monitor, config).await;
        });
    }

    println!("geber \"{}\" is running", config.name);
    println!("reporting to {}\n", config.upstream);
    println!("Press CTRL-C to exit.\n");

    let _ = tokio::signal::ctrl_c().await;


}

async fn perfom_uptime_check(monitor: Monitor, config: Config) {
    let client = reqwest::Client::new();

    loop {
        println!("performing uptime check {} {}", monitor.slug, monitor.url);

        let res = client.get(&monitor.url).send().await;

        tokio::spawn(report_uptime_check(res, monitor.clone(), config.clone()));
        tokio::time::sleep(Duration::from_secs(monitor.frequency)).await;
    }
}

async fn report_uptime_check(
    response: Result<reqwest::Response, reqwest::Error>,
    monitor: Monitor,
    config: Config,
) {
    let http_status = match response {
        Ok(res) => res.status(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let json = json!({
        "monitor_slug": monitor.slug,
        "http_status": http_status.as_u16(),
    });

    println!("reporting uptime check {} with status {}", monitor.slug, http_status);

    let client = reqwest::Client::new();
    let res = client.post(&config.upstream)
        .bearer_auth(config.api_token)
        .json(&json)
        .send()
        .await;

    let status = match res {
        Ok(res) => res.status(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    println!("reported uptime check, got {} response", status.as_str());
}
