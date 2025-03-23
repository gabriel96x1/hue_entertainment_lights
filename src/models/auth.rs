use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyGenerationSuccess {
    pub username: String,
    pub clientkey: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HueBridgeResponse {
    pub success: KeyGenerationSuccess
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestKeyBody {
    pub devicetype: String,
    pub generateclientkey: bool,
}

#[derive(Debug)]
pub struct ConnectionData {
    pub app_id: String,
    pub app_key: String,
    pub client_key: String,
}