use serde::{Serialize, Deserialize};
use frost_ed25519::Signature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningRequest {
    pub message: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningResponse {
    pub signature: Vec<u8>,
}
