use serde::{Deserialize, Serialize};

pub type KeyId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawProtocolMessage {
    pub session_id: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum Command {}
