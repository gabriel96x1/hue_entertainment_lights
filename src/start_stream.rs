use std::collections::HashMap;
use std::future::Future;
use log::info;
use reqwest::Client;

pub async fn start_stream(client_key: &str, bridge_ip: &str, reqwest_client: &Client) {
    let url_start_stream = format!("https://{}/clip/v2/entertainment_configuration/", bridge_ip);
    let mut start_params = HashMap::new();
    start_params.insert("action","start");

    let response = reqwest_client.put(url_start_stream).header("hue-application-key", client_key)
        .form(&start_params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    info!("start stream response: {response}");
}