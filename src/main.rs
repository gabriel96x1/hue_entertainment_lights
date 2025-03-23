mod setup_stream;
mod start_stream;
mod models;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use log::info;
use reqwest::Certificate;
use crate::setup_stream::setup_stream;
use crate::start_stream::start_stream;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args: Vec<String> = env::args().collect();
    let bridge_ip = &args[1];

    let mut buf = Vec::new();
    let relative_cert_path = Path::new("cert.pem");
    let cert_exists = relative_cert_path.exists();
    info!("Certificate exists: {cert_exists}");

    let _ = File::open(relative_cert_path).unwrap()
        .read_to_end(&mut buf);
    //let cert = Certificate::from_pem(&buf).unwrap();

    info!("Connecting with Hue Bridge at {bridge_ip}");
    let req_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let connection_data = setup_stream(&bridge_ip, &req_client);
    //info!("Connection Data: {}", connection_data.);
    //start_stream(&client_key, &bridge_ip, &req_client);
}
