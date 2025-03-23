use std::collections::HashSet;
use std::fmt::Debug;
use crate::models;

use log::{error, info};
use reqwest::{Client, StatusCode};
use crate::models::auth::{ConnectionData, HueBridgeResponse, RequestKeyBody};

#[tokio::main]
pub async fn setup_stream(bridge_ip: &str, reqwest_client: &Client) -> ConnectionData {

    let mut app_id = String::new();
    let mut app_key = String::new();
    let mut client_key = String::new();

    let body = RequestKeyBody {
        devicetype: String::from("app_name#instance_name"),
        generateclientkey: true,
    };
    let url = format!("https://{}/api", bridge_ip);

    let response = reqwest_client.post(url)
        .json(&body)
        .send()
        .await
        .unwrap();


    match response.status() {
        StatusCode::OK => {
            match response.text().await {
                Ok(value) => {
                    info!("{}", value);

                    let parsed: Vec<HueBridgeResponse> = serde_json::from_str(&value)
                        .unwrap();

                    app_key = parsed.get(0).unwrap().success.username.clone();
                    client_key = parsed.get(0).unwrap().success.clientkey.clone();

                }
                Err(e) => {
                    error!("Error: {:?}", e.to_string());
                }
            }
        }
        other => {
            error!("StatusCode: {:?}", other.to_string());
        }
    }

    let url_auth = format!("https://{}/auth/v1", bridge_ip);

    let app_id_response = reqwest_client.get(url_auth)
        .header("hue-application-key", app_key.clone())
        .send()
        .await
        .unwrap();

    match app_id_response.status() {
        StatusCode::OK => {
            match app_id_response.text().await {
                Ok(value) => {
                    info!("{}", value);

                    let parsed: Vec<HueBridgeResponse> = serde_json::from_str(&value)
                        .unwrap();

                    app_id = value

                }
                Err(e) => {
                    error!("Error: {:?}", e.to_string());
                }
            }
        }
        other => {
            error!("StatusCode: {:?}", other.to_string());
        }
    }

    ConnectionData {
        app_id,
        app_key,
        client_key
    }
}
