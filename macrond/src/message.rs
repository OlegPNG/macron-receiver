use serde::{Serialize, Deserialize};

use crate::MacronFunction;

#[derive(Serialize, Deserialize)]
pub struct AuthMessage {
    #[serde(rename="type")]
    pub message_type: String,
    pub session_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutboundMessage {
    #[serde(rename="type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub receiver_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<MacronFunction>>,
}

#[derive(Serialize, Deserialize)]
pub struct InboundMessage {
    #[serde(rename="type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
}
