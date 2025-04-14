#[warn(dead_code)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
mod client;
use libp2p::{
    gossipsub, mdns,
    rendezvous::{self, Cookie},
    swarm::{behaviour::toggle::Toggle, NetworkBehaviour},
};
use tokio::sync::mpsc;
pub type PeerId = libp2p::PeerId;

pub use crate::client::*;
#[derive(Debug)]
pub enum Event {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolExecutionParams {
    pub protocol: String,
    pub key_id: KeyId,
    pub peer_ids: Vec<PeerId>,
    pub data: Option<Vec<u8>>,
}
#[derive(NetworkBehaviour)]
struct _NodeBehavior {
    ping: libp2p::ping::Behaviour,
    rendezvous_server: Toggle<rendezvous::server::Behaviour>,
    rendezvous_client: Toggle<rendezvous::client::Behaviour>,
    gossipsub: gossipsub::Behaviour,
    mdns: Toggle<mdns::tokio::Behaviour>,
}

pub struct Node {
    #[allow(dead_code)]
    swarm: String,
    #[allow(dead_code)]
    rendezvous_point: Option<PeerId>,
    #[allow(dead_code)]
    rendezvous_cookie: Option<Cookie>,
    #[allow(dead_code)]
    message_sender: mpsc::UnboundedSender<RawProtocolMessage>,
}

impl Node {
    pub fn new(
        _command_receiver: mpsc::UnboundedReceiver<Command>,
        message_sender: mpsc::UnboundedSender<RawProtocolMessage>,
        _event_sender: mpsc::UnboundedSender<Event>,
    ) -> Result<Self> {
        Ok(Self {
            swarm: String::new(),
            rendezvous_point: None,
            rendezvous_cookie: None,
            message_sender,
        })
    }
}
